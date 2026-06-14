import { useState, useEffect, useContext } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../App.css";
import { AppContext } from "../AppContext";

function LiveviewTab() {
    const { appData, setAppData, pregameData, setPregameData } = useContext(AppContext);
    const { userData, setUserData} = useState({
        agent: {
            name: null,
            icon: null,
            id: null
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
    });
    
    return ["INGAME", "PREGAME"].includes(appData.gamestate) ?
    (
        <div className="animate-[container-opacity-enter_0.5s_forwards]">
            <h1>{appData.gamestate} {JSON.stringify(pregameData)}</h1>
        </div>
    ) :
    
    (
        <div className="flex animate-[container-opacity-enter_0.5s_forwards] text-[5vh] w-full flex-1">
            <h1 className="flex-1">Waiting for next match</h1>
        </div>
    );
}

export default LiveviewTab;