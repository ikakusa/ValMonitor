import { useState, useEffect, createContext } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

import HomeTab from "./Tabs/HomeTab";
import LiveviewTab from "./Tabs/LiveviewTab";
import SettingsTab from "./Tabs/SettingsTab";

import { AppContext } from "./AppContext";

function App() {
    const [appData, setAppData] = useState({
        auth_info: {},
        my_presence: {},
        current_page: "home",
        presence: {},
        gamestate: "OFFLINE",
        puuid: "",
        full_name: "",
    });

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

    useEffect(() => {
        async function load_userinfo() {
            await invoke("get_auth_userinfo").then((json) => {
                try {
                    const parsed = JSON.parse(json);
                    setAppData(prev => ({ ...prev, auth_info: parsed }))
                } catch { }
            }).catch(err => { });
        }

        async function get_gamestate() {
            await invoke("get_gamestate").then((json) => {
                setAppData(prev => ({ ...prev, gamestate: json }))
            });
        }

        async function load_my_presence() {
            try {
                await invoke("get_private_presence").then((json) => {
                    try {
                        const parsed = JSON.parse(json);
                        setAppData(prev => ({ ...prev, presence: parsed }))
                    } catch {

                    }
                })
                    .catch(err => { });

                const full_name = await invoke("get_full_username");
                const puuid = await invoke("get_puuid");

                setAppData(prev => ({
                    ...prev,
                    full_name,
                    puuid
                }));
            } catch {

            }
        }


        const interval = setInterval(() => {
            get_gamestate();
            load_my_presence();
        }, 1000);
        return () => clearInterval(interval);
    }, []);
    function getActivated(input) {
        return input === appData.current_page ? "is_active" : "";
    }

    return (
        <AppContext.Provider
            value={{ appData, setAppData }}
        >
            <main className="container">
                <div className="main_tab">
                    <div className={`btn home ${getActivated("home")}`} onClick={() => setAppData(prev => ({ ...prev, current_page: "home" }))}><a>Home</a></div>
                    <div className={`btn liveview ${getActivated("liveview")}`} onClick={() => setAppData(prev => ({ ...prev, current_page: "liveview" }))}><a>Live view</a></div>
                    <div className={`btn settings ${getActivated("settings")}`} onClick={() => setAppData(prev => ({ ...prev, current_page: "settings" }))}><a>Settings</a></div>
                    <div className={`indicator ${appData.current_page}`}></div>
                </div>
                {appData.current_page === "home" && <HomeTab />}
                {appData.current_page === "liveview" && <LiveviewTab />}
                {appData.current_page === "settings" && <SettingsTab />}
            </main>
        </AppContext.Provider>
    );

    // const [greetMsg, setGreetMsg] = useState("");
    // const [name, setName] = useState("");

    // async function greet() {
    //     // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    //     setGreetMsg(await invoke("greet", { name }));
    // }

    // return (
    //     <main className="container">
    //         <h1>Welcome to Tauri + React</h1>

    //         <div className="row">
    //             <a href="https://vite.dev" target="_blank">
    //                 <img src="/vite.svg" className="logo vite" alt="Vite logo" />
    //             </a>
    //             <a href="https://tauri.app" target="_blank">
    //                 <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
    //             </a>
    //             <a href="https://react.dev" target="_blank">
    //                 <img src={reactLogo} className="logo react" alt="React logo" />
    //             </a>
    //         </div>
    //         <p>Click on the Tauri, Vite, and React logos to learn more.</p>

    //         <form
    //             className="row"
    //             onSubmit={(e) => {
    //                 e.preventDefault();
    //                 greet();
    //             }}
    //         >
    //             <input
    //                 id="greet-input"
    //                 onChange={(e) => setName(e.currentTarget.value)}
    //                 placeholder="Enter a name..."
    //             />
    //             <button type="submit">Greet</button>
    //         </form>
    //         <p>{greetMsg}</p>
    //     </main>
    // );
}

export default App;
