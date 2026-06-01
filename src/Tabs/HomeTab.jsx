import { useState, useEffect, useContext } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../App.css";
import { AppContext } from "../AppContext";

function HomeTab() {
    const { appData, setAppData } = useContext(AppContext);
    return (
        <div className="home_container">
            <h1>{appData.full_name} {appData.puuid} {JSON.stringify(appData.gamestate)}</h1>
        </div>
    );
}

export default HomeTab;