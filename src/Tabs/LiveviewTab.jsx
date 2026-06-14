import { useState, useEffect, useContext } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../App.css";
import { AppContext } from "../AppContext";

function LiveviewTab() {
    const { appData, setAppData, matchData, userData } = useContext(AppContext);

    useEffect(() => {
    }, [matchData, userData]);
    
    return ["INGAME", "PREGAME"].includes(appData.gamestate) ?
    (
        <div className="animate-[container-opacity-enter_0.5s_forwards]">
            <h1>{appData.gamestate}</h1>
            {Object.entries(userData).map(([key, value]) => { return <h1>{`${value.agent.name ?? "None"} - ${value.name}\n`}</h1> } )}
        </div>
    ) :
    
    (
        <div className="flex animate-[container-opacity-enter_0.5s_forwards] text-[5vh] w-full flex-1">
            <h1 className="flex-1">Waiting for next match</h1>
        </div>
    );
}

export default LiveviewTab;