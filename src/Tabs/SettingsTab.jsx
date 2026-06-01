import { useState, useEffect, useContext } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../App.css";
import { AppContext } from "../AppContext";

function SettingsTab() {
    const { appData, setAppData } = useContext(AppContext);
    return (
        <div className="settings_container"></div>
    );
}

export default SettingsTab;