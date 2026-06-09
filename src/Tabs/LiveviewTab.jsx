import { useState, useEffect, useContext } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../App.css";
import { AppContext } from "../AppContext";

function LiveviewTab() {
    const { appData, setAppData } = useContext(AppContext);
    return (
        <div className="animate-[container-opacity-enter_0.5s_forwards]">
            <h1>何見てんだおめえ</h1>
        </div>
    );
}

export default LiveviewTab;