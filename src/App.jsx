import { useState, useEffect, createContext } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

import HomeTab from "./Tabs/HomeTab";
import LiveviewTab from "./Tabs/LiveviewTab";
import SettingsTab from "./Tabs/SettingsTab";

import { AppContext } from "./AppContext";

function App() {
    const [competitive_seasons, setCompetitiveSeasons] = useState([]);
    const [appData, setAppData] = useState({
        auth_info: {},
        my_presence: {},
        current_page: "home",
        presence: {},
        gamestate: "IDLE",
        puuid: null,
        full_name: null,
        card_id: null,
        account_level: 0,
        ranks: [],
        card_image: null,
        current_season_id: null,
        current_season_name: null,
        current_season_peak: 0,
        region: null,
        peak_rank: { tier: 0, season_id: null, season_name: null, icon: null, name: "Unranked", rr: 0 },
        current_rank: { tier: 0, icon: null, name: "Unranked" }
    });

    useEffect(() => {
        document.addEventListener('contextmenu', function (event) {
            event.preventDefault();
        });

        document.addEventListener('keydown', (event) => {
            if (
                event.key === 'F5' ||
                (event.ctrlKey && event.key.toLowerCase() === 'r') ||
                (event.metaKey && event.key.toLowerCase() === 'r')
            ) {
                // event.preventDefault();
            }
        });
        async function load_userinfo() {
            await invoke("get_auth_userinfo").then((json) => {
                setAppData(prev => ({ ...prev, auth_info: json }))
            }).catch(err => { });
        }

        async function get_gamestate() {
            await invoke("get_gamestate").then((json) => {
                setAppData(prev => ({ ...prev, gamestate: json }));
            }).catch();
        }

        async function get_my_card(presence) {
            return presence["playerPresenceData"]["playerCardId"];
        }

        async function get_my_level(presence) {
            return presence["playerPresenceData"]["accountLevel"];
        }

        async function get_region() {
            return await invoke("get_region");
        }

        async function load_my_presence() {
            try {
                const json = await invoke("get_private_presence");

                const full_name = await invoke("get_full_username");
                const puuid = await invoke("get_puuid");
                const level = await get_my_level(json);
                const card_id = await get_my_card(json);
                const region = await get_region();

                if (appData.card_id !== card_id) {
                    const card_json = await invoke("get_playercard_by_id", { id: card_id });
                    setAppData(prev => ({ ...prev, card_id: card_id, card_image: card_json["displayIcon"] }));
                }

                setAppData(prev => ({
                    ...prev,
                    presence: json,
                    full_name,
                    puuid,
                    region: region,
                    account_level: level
                }));
            } catch (e) {
                console.log(e)
            }
        }

        async function load_ranks() {
            const data = await (await fetch("https://valorant-api.com/v1/competitivetiers")).json();
            const ranks = data["data"][data["data"].length - 1]["tiers"];
            setAppData(prev => ({
                ...prev,
                ranks: ranks
            }));
        }

        async function fetch_seasons() {
            setCompetitiveSeasons((await (await fetch("https://valorant-api.com/v1/seasons")).json())["data"]);
        }


        fetch_seasons();
        get_gamestate();
        load_my_presence();
        load_ranks();

        const interval = setInterval(() => {
            get_gamestate();
            load_my_presence();
        }, 1000);
        return () => clearInterval(interval);
    }, []);

    useEffect(() => {
        if (!appData.puuid || !appData.ranks || competitive_seasons.length === 0) return;

        async function get_rank() {
            const json = await invoke("get_player_mmr", { uid: appData.puuid });
            console.log(json)
            if (!json["QueueSkills"]) return;
            try {
                const all_seasons = competitive_seasons;
                const now = Date.now();
                const _current_season = all_seasons.filter(season => {
                    const start = new Date(season.startTime).getTime();
                    const end = new Date(season.endTime).getTime();

                    return now >= start && now <= end && season["title"];
                });
                if (_current_season.length <= 0) return;
                const current_season = _current_season[0];
                const current_season_id = current_season["uuid"];
                const current_season_name = current_season["title"];

                const competitive = json["QueueSkills"]["competitive"];
                const seasons = competitive["SeasonalInfoBySeasonID"];

                const current_competitive = seasons[current_season_id];
                if (!current_competitive) return;

                const peak_rank_entries = Object.entries(seasons).sort((a, b) => {
                    return b[1]["CompetitiveTier"] - a[1]["CompetitiveTier"];
                });
                const [peak_season_id, peak_rank] = peak_rank_entries[0];

                const peak_rank_tier = peak_rank["CompetitiveTier"];
                const peak_rank_icon = appData.ranks[peak_rank_tier]["largeIcon"];
                const peak_rank_name = appData.ranks[peak_rank_tier]["tierName"];

                const current_rank_tier = current_competitive["CompetitiveTier"];
                const current_rr = current_competitive["RankedRating"];
                const current_peak = current_competitive["Rank"];

                const current_rank_name = appData.ranks[current_rank_tier]["tierName"];
                const current_rank_icon = appData.ranks[current_rank_tier]["largeIcon"];

                const season_peak_rank_name = appData.ranks[current_peak]["tierName"];
                const season_peak_icon = appData.ranks[current_peak]["largeIcon"];

                const peak_season = (await (await fetch(`https://valorant-api.com/v1/seasons/${peak_season_id}`)).json())["data"];
                const peak_season_title = peak_season["title"];

                setAppData(prev => ({
                    ...prev,
                    current_season_id: current_season_id,
                    current_season_name: current_season_name,
                    current_season_peak: { tier: current_peak ?? 0, icon: season_peak_icon, name: season_peak_rank_name.charAt(0) + season_peak_rank_name.slice(1).toLowerCase() },
                    current_rank: { tier: current_rank_tier ?? 0, icon: current_rank_icon, name: current_rank_name.charAt(0) + current_rank_name.slice(1).toLowerCase(), rr: current_rr ?? 0 },
                    peak_rank: { season_id: peak_season_id, season_name: peak_season_title, tier: peak_rank_tier ?? 0, icon: peak_rank_icon, name: peak_rank_name.charAt(0) + peak_rank_name.slice(1).toLowerCase() }
                }));
            } catch (e) {
                console.log(e)
            }
        }

        get_rank();
        const interval = setInterval(() => {
            get_rank();
        }, 1000 * 60);
        return () => clearInterval(interval);
    }, [appData.puuid, appData.ranks, competitive_seasons]);
    function getActivated(input) {
        return input === appData.current_page ? "is_active" : "";
    }

    return (
        <AppContext.Provider value={{ appData, setAppData }}>
            <main className="flex flex-col justify-center text-center">
                <div className="relative flex h-[8vh] w-full bg-[#252525]">
                    <div
                        className={`flex flex-1 cursor-pointer items-center justify-center transition-all duration-300 ${appData.current_page === "home"
                                ? "bg-[#2f2f2f]"
                                : "hover:bg-[#2a2a2a]"
                            }`}
                        onClick={() =>
                            setAppData(prev => ({
                                ...prev,
                                current_page: "home",
                            }))
                        }
                    >
                        <span className="font-medium text-white">
                            Home
                        </span>
                    </div>

                    <div
                        className={`flex flex-1 cursor-pointer items-center justify-center transition-all duration-300 ${appData.current_page === "liveview"
                                ? "bg-[#2f2f2f]"
                                : "hover:bg-[#2a2a2a]"
                            }`}
                        onClick={() =>
                            setAppData(prev => ({
                                ...prev,
                                current_page: "liveview",
                            }))
                        }
                    >
                        <span className="font-medium text-white">
                            Live view
                        </span>
                    </div>

                    <div
                        className={`flex flex-1 cursor-pointer items-center justify-center transition-all duration-300 ${appData.current_page === "settings"
                                ? "bg-[#2f2f2f]"
                                : "hover:bg-[#2a2a2a]"
                            }`}
                        onClick={() =>
                            setAppData(prev => ({
                                ...prev,
                                current_page: "settings",
                            }))
                        }
                    >
                        <span className="font-medium text-white">
                            Settings
                        </span>
                    </div>

                    <div
                        className={`absolute bottom-0 left-0 h-0.5 w-1/3 bg-red-500 transition-transform duration-100 ${appData.current_page === "home"
                                ? "translate-x-0"
                                : appData.current_page === "liveview"
                                    ? "translate-x-full"
                                    : "translate-x-[200%]"
                            }`}
                    />
                </div>

                {appData.current_page === "home" && <HomeTab />}
                {appData.current_page === "liveview" && <LiveviewTab />}
                {appData.current_page === "settings" && <SettingsTab />}
            </main>
        </AppContext.Provider>
    );
}

export default App;
