import { useState, useEffect, useContext } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../App.css";
import { AppContext } from "../AppContext";

function SettingsTab() {
    const { appData, setAppData } = useContext(AppContext);
    return (
        <div className="animate-[container-opacity-enter_0.5s_forwards]">
            <h1>こっちみんな</h1>
        </div>
    );
}

export default SettingsTab;