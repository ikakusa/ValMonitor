import { useState, useEffect, useContext } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../App.css";
import { AppContext } from "../AppContext";
import valorant_icon from "../Assets/valorant_icon.png";
import unranked_icon from "../Assets/unranked.png";

function RankPanel({
    title_text,
    season_name,
    tier,
    rr,
    icon,
    no_rr = false,
}) {
    return (
        <div>
            <div className="flex justify-between">
                <p className="ml-2.5 mt-2.5 mb-1 text-left text-[1.25em]">
                    {title_text}
                </p>

                <p className="mr-2.5 mt-5 text-right text-[0.85em] text-gray-500">
                    {season_name}
                </p>
            </div>

            <div className="mx-1.25 mt-[-2.5px] mb-1.25 flex h-[13.5vh] w-[28.5vw] items-center rounded-2xl bg-[#222222]">
                <img
                    src={icon ?? unranked_icon}
                    className="ml-1.25 h-[10vh]"
                    alt=""
                />

                <div className="ml-1.25 flex flex-col gap-[1.5px] text-left text-[1.35em]">
                    <p className="m-0">{tier}</p>

                    {!no_rr && (
                        <p className="m-0 text-[0.8em]">
                            {rr} RR
                        </p>
                    )}
                </div>
            </div>
        </div>
    );
}

function HomeTab() {
    const { appData } = useContext(AppContext);

    return (
        <div className="animate-[container-opacity-enter_0.5s_forwards]">
            <div className="flex h-[15vh] w-full bg-[#222222]">
                <div className="ml-4 flex h-[15vh] w-full items-center text-left">
                    <img
                        className="h-16 w-16 rounded-full border-2 border-white"
                        src={appData.card_image ?? valorant_icon}
                        alt=""
                    />

                    <div className="ml-2 flex flex-col gap-1.25 text-[1.25em]">
                        <p className="m-0">
                            {appData.full_name}
                        </p>

                        <p className="m-0 text-[0.85em] text-gray-500">
                            Level: {appData.account_level}
                        </p>
                    </div>
                </div>
            </div>

            <div className="flex flex-row">
                <div>
                    <RankPanel
                        title_text="Current Rank"
                        season_name={appData.current_season_name}
                        tier={appData.current_rank.name}
                        icon={appData.current_rank.icon}
                        rr={appData.current_rank.rr}
                    />

                    <RankPanel
                        title_text="Season Peak Rank"
                        season_name={appData.current_season_name}
                        tier={appData.current_season_peak.name}
                        icon={appData.current_season_peak.icon}
                        no_rr
                    />

                    <RankPanel
                        title_text="Peak Rank"
                        season_name={appData.peak_rank.season_name}
                        tier={appData.peak_rank.name}
                        icon={appData.peak_rank.icon}
                        no_rr
                    />
                </div>

                <div className="flex w-[2vw] justify-center">
                    <div className="mx-1.25 my-2.5 w-px bg-[#3f3f3f]" />
                </div>

                <div>
                    <p className="ml-2.5 mt-2.5 mb-1.25 text-left text-[1.25em]">
                        Status
                    </p>

                    <div className="ml-[2.5px] mr-1.25 mt-[-2.5px] flex h-[68vh] w-[67vw] rounded-2xl bg-[#222222]" />
                </div>
            </div>
        </div>
    );
}

export default HomeTab;