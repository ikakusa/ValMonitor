import { useState, useEffect, useContext } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../App.css";
import { AppContext } from "../AppContext";
import valorant_icon from "../Assets/valorant_icon.png";
import unranked_icon from "../Assets/unranked.png";

function RankPanel({ id, title_text, season_name, tier, rr, icon, no_rr = false }) {
    return <div className={id}>
        <div id="title_panel">
            <p id="current_rank">{title_text}</p>
            <p id="season_name">{season_name}</p>
        </div>
        <div id="rank_element">
            <img id="rank_icon" src={icon ?? unranked_icon}></img>
            <div id="rank_text">
                <p id="name">{tier}</p>
                {!no_rr && <p id="rr">{rr} RR</p>}
            </div>
        </div>
    </div>
}

function HomeTab() {
    const { appData, setAppData } = useContext(AppContext);
    return (
        <div className="home_container">
            <div className="user_info_panel">
                <div className="user_info_padding">
                    <img className="player_icon" src={appData.card_image ?? valorant_icon}></img>
                    <div className="name_panel">
                        <p id="p_name">{appData.full_name}</p>
                        <p id="p_level">Level: {appData.account_level}</p>
                    </div>
                </div>
            </div>
            <div className="info_container">
                <div className="rank_info_panel">
                    <RankPanel
                        id="current_rank_panel"
                        title_text="Current Rank"
                        season_name={appData.current_season_name}
                        tier={appData.current_rank.name}
                        icon={appData.current_rank.icon}
                        rr={appData.current_rank.rr}
                    />
                    <RankPanel
                        id="current_peak_rank_panel"
                        title_text="Season Peak Rank"
                        season_name={appData.current_season_name}
                        tier={appData.current_season_peak.name}
                        icon={appData.current_season_peak.icon}
                        no_rr
                    />
                    <RankPanel
                        id="peak_rank_panel"
                        title_text="Peak Rank"
                        season_name={appData.peak_rank.season_name}
                        tier={appData.peak_rank.name}
                        icon={appData.peak_rank.icon}
                        no_rr
                    />
                </div>

                <div className="rank_panel_divider">
                    <div id="divider">

                    </div>
                </div>

                <div className="stats_panel_container">
                    <p id="status">Status</p>
                    <div className="stats_panel">
                    </div>
                </div>
            </div>
        </div>
    );
}

export default HomeTab;