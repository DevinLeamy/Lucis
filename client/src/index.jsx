import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom/client";
import { ElementDisplay } from "./element_display"
import { Switch } from "@mui/material";
import { createTheme, ThemeProvider } from '@mui/material/styles';
import { loadWasm, WorkerPool } from "./wasm_loader";
import { ConfigContextProvider } from "./contexts/config";

import "./index.css";
import "./styles.css";

loadWasm(main);

const Element = {
  id: { id: 6559696101191670000 },
  material: {
    Metal: {
      texture: { SolidTexture: { color: { red: 0.2, green: 0.2, blue: 0.9 } } },
      fuzz: 0.2,
    },
  },
  shape: { Sphere: { center: { x: 0.0, y: 0.0, z: 0.0 }, radius: 0.5 } },
}; 

const theme = createTheme({
    typography: {
        fontSize: 10 
    },
})

function main(wasm) {
    const App = () => {
        const [element, setElement] = useState(Element);

        useEffect(() => {
            setElement(Element);
        }, [])

   

        const onElementUpdate = (element) => {
            setElement(element);
        }

        return (
            <div>
                <div className="app-header" position="static">
                    Ray Tracer Playground
                </div>
                <Switch label="Element View" />
                <ElementDisplay 
                    element={element} 
                    onElementUpdate={onElementUpdate} 
                />
            </div>
        )
    };

    const root = ReactDOM.createRoot(document.getElementById("root"));
    root.render(
        <React.StrictMode>
            <ThemeProvider theme={theme}>
                <ConfigContextProvider>
                    <App />
                </ConfigContextProvider>
            </ThemeProvider>
            <button onClick={() => wasm.marco()}>Marco</button>
        </React.StrictMode>
    );
}
