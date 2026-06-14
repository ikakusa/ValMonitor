import { useState, useEffect, createContext } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

import HomeTab from "./Tabs/HomeTab";
import LiveviewTab from "./Tabs/LiveviewTab";
import SettingsTab from "./Tabs/SettingsTab";

import { AppContext } from "./AppContext";

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

const defaultPlayer = {
    agent: {
        name: null,
        icon: null,
        id: null,
        locked: false
    },
    rank: {
        rr: null,
        tier: null,
        peak_tier: null,
        icon: null,
        peak_icon: null
    },
    stats: {
        match_count: null,
        dmr: null,
        kda: {
            k: null,
            d: null,
            a: null
        },
        win_rate: null,
        hs: null
    },
    name: null,
    puuid: null,
    /**
     * { weapon_id, weapon_name, weapon_image }
     */
    skins: [],
}

const api_wait_time = 750;

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
    const [pregameData, setPregameData] = useState({});
    const [ingameMatchData, setIngameMatchData] = useState({});
    const [matchData, setMatchData] = useState({});

    const [userData, setUserData] = useState({});

    async function get_pregame() {
        return await invoke("get_current_pregame");
    }

    async function get_ingame_match() {
        return await invoke("get_current_game");
    }

    //pregameとingame時にユーザーデータを取ります stats rank 名前 エージェント(lockedの場合)はキャッシュされて引き継がれます
    async function prepare_user_data() {
        async function resolve_player_name(puuid, fallback) {
            const info = await invoke("get_player_by_id", { id: puuid });
            return info.status === 200 ? `${info.data.name}#${info.data.tag}` : fallback;
        }
        async function update_user_data(default_data) {
            console.log(default_data);
            const puuid = default_data.puuid;
            setUserData(prev => {
                const next = {
                    ...(prev[puuid] || {}),
                    ...default_data,
                    agent: {
                        ...(prev[puuid]?.agent || {}),
                        ...default_data.agent
                    },
                    rank: {
                        ...(prev[puuid]?.rank || {}),
                        ...default_data.rank
                    },
                    stats: {
                        ...(prev[puuid]?.stats || {}),
                        ...default_data.stats
                    }
                };

                if (JSON.stringify(next) === JSON.stringify(prev[puuid])) {
                    return prev;
                }
                console.log("update user data");

                return {
                    ...prev,
                    [puuid]: next
                };
            });
        }
        async function handle_ingame() {
            const data = matchData["data"];
            for (const player of data["Players"]) {
                const default_data = structuredClone(defaultPlayer);
                const puuid = player["Subject"];
                const cached_me = userData ? userData[puuid] ?? null : null;

                default_data.puuid = puuid;
                let agent_name = cached_me && cached_me["agent"]["locked"] ? cached_me["agent"]["name"] : "None";
                if (!cached_me || !cached_me["agent"]["locked"]) {
                    const _agent_data = await invoke("get_agent_by_id", { id: player["CharacterID"] });
                    const agent_data = _agent_data["data"];
                    agent_name = agent_data["displayName"] ?? "None";
                    default_data.agent = {
                        icon: agent_data["displayIcon"] ?? null,
                        id: player["CharacterID"] ?? null,
                        name: agent_data["displayName"] ?? "None",
                        locked: true
                    };
                }
                if (!cached_me?.name) {
                    const player_info = await invoke("get_player_by_id", { id: player["Subject"] });
                    default_data.name = resolve_player_name(agent_name);
                }
                update_user_data(default_data);
                await sleep(api_wait_time);
            }
        }
        async function handle_pregame() {
            const data = matchData["data"];
            for (const player of data["AllyTeam"]["Players"]) {
                const default_data = structuredClone(defaultPlayer);
                const puuid = player["Subject"];
                const cached_me = userData ? userData[puuid] ?? null : null;
                default_data.puuid = puuid;
                default_data.agent = cached_me?.agent;
                if (player["CharacterID"] && !default_data.agent.locked) {
                    const _agent_data = await invoke("get_agent_by_id", { id: player["CharacterID"] });
                    const agent_data = _agent_data["data"];
                    const agent_name = agent_data["displayName"] ?? "None";
                    default_data.agent = {
                        icon: agent_data["displayIcon"] ?? null,
                        id: player["CharacterID"] ?? null,
                        name: agent_name,
                        locked: player["CharacterSelectionState"] === "locked"
                    };
                }
                default_data.name = cached_me?.name;
                if (!default_data.name) {
                    const player_info = await invoke("get_player_by_id", { id: player["Subject"] });
                    default_data.name = resolve_player_name(agent_name);
                }
                update_user_data(default_data);
                await sleep(api_wait_time);
            }
        }
        switch (appData.gamestate) {
            case "INGAME": await handle_ingame(); break
            case "PREGAME": await handle_pregame(); break
        }
    }

    //PREGAME -> INGAME以外の場合にuserdataをリセット
    useEffect(() => {
        if (
            appData.gamestate !== "PREGAME" &&
            appData.gamestate !== "INGAME"
        ) {
            setUserData({});
        }
    }, [appData.gamestate]);

    //ページロード時にcontextmenuの無効化、各種ループの登録
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
                setAppData(prev => {
                    if (prev.gamestate === json)
                        return prev;
                    return { ...prev, gamestate: json };
                });
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
                    const card_json = (await invoke("get_playercard_by_id", { id: card_id }))["data"];
                    setAppData(prev => ({ ...prev, card_id: card_id, card_image: card_json["displayIcon"] }));
                }

                setAppData(prev => ({
                    ...prev,
                    presence: json,
                    full_name: full_name,
                    puuid: puuid,
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
        return () => { clearInterval(interval); };
    }, []);

    //ループでpregameDataかingameDataをmatchDataにいれる
    useEffect(() => {
        const interval = setInterval(() => {
            let newData = null;
            switch (appData.gamestate) {
                case "PREGAME": {
                    newData = { state: appData.gamestate, data: pregameData };
                } break;
                case "INGAME": {
                    newData = { state: appData.gamestate, data: ingameMatchData };
                } break;
            }
            if (newData) {
                setMatchData(prev => {
                    if (JSON.stringify(prev) !== JSON.stringify(newData) && newData["data"]["ID"])
                        return newData;
                    return prev;
                });
            }
        }, 500);
        return () => {
            clearInterval(interval);
        }
    }, [pregameData, ingameMatchData, appData.gamestate]);

    //１分毎にローカルプレイヤーのランクを取得するループ
    useEffect(() => {
        if (!appData.puuid || !appData.ranks || competitive_seasons.length === 0) return;

        async function get_rank() {
            const json = await invoke("get_player_mmr", { uid: appData.puuid });
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

        return () => {
            clearInterval(interval);
        }
    }, [appData.puuid, appData.ranks, competitive_seasons]);

    //gamestateをもとに各種matchDataの更新
    useEffect(() => {
        async function tick_gamedata() {
            async function set_ingame_matchdata() {
                if (appData.gamestate !== "INGAME") {
                    return;
                }
                await get_ingame_match().then(res => {
                    setIngameMatchData(prev => {
                        if (JSON.stringify(prev) === JSON.stringify(res) || !res.MatchID) {
                            return prev;
                        }

                        console.log("update match data");
                        return { ...res, ID: res.MatchID };
                    });
                });
            }
            async function set_pregame() {
                if (appData.gamestate !== "PREGAME") {
                    return;
                }
                get_pregame().then(res => {
                    setPregameData(prev => {
                        if (JSON.stringify(prev) === JSON.stringify(res) || !res.ID) {
                            return prev;
                        }

                        console.log("update pregame");
                        return res;
                    });
                });
            }
            set_pregame();
            set_ingame_matchdata();
        }
        tick_gamedata();
        const pregame_interval = setInterval(async () => {
            tick_gamedata();
        }, 10000);
        return () => {
            clearInterval(pregame_interval);
        }
    }, [appData.gamestate, tick_gamedata]);

    //いわんでもわかるだろ
    useEffect(() => {
        prepare_user_data();
    }, [matchData]);

    return (
        <AppContext.Provider value={{ appData, setAppData, matchData, setMatchData, userData }}>
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
