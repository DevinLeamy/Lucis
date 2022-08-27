import { ReactJSXElement } from "@emotion/react/types/jsx-namespace";
import { Button, FormControl, Card, FormControlLabel, FormLabel, Paper, Radio, RadioGroup, Slider } from "@mui/material";
import React, { ReactNode, useContext, useEffect, useRef, useState } from "react"; 
import { RequestEmitter, WorkerPool } from "./wasm_loader";
import { MaterialDisplay } from "./material_display";
import { ShapeDisplay } from "./shape_display";
import { CameraDisplay } from "./camera_display";
import { QualityDisplay } from "./quality_display";

import "./styles.css";
import { ConfigContext } from "./contexts/config";

interface ElementDisplayProps {
    element: any,
    onElementUpdate: (element) => {} 
}

let threadCount = 5; // navigator.hardwareConcurrency;

const DEFAULT_BACKGROUND_MATERIAL = {
    Lambertian: {
        texture: { 
            CheckeredTexture: { 
                even: { red: 0.0, green: 0.0, blue: 0.0 },
                odd:  { red: 1.0, green: 1.0, blue: 1.0 } 
            } 
        }
    },
}

const ElementDisplay = ({ element, onElementUpdate }: ElementDisplayProps) => {
    const { config } = useContext(ConfigContext)
    const [pool, setPool] = useState<typeof WorkerPool>(undefined); 
    const [framesRendering, setFramesRendering] = useState(0);
    const [canvasImageURL, setCanvasImageURL] = useState<string>(undefined);
    const [backgroundMat, setBackgroundMat] = useState<any>(DEFAULT_BACKGROUND_MATERIAL);

    console.log("Element", element)

    let requestEmitter = new RequestEmitter();

    const canvasRef = useRef<HTMLCanvasElement>(null);

    const { material, shape } = element;

    useEffect(() => {
        if (pool === undefined) {
            let newPool = new WorkerPool(threadCount);

            render(element, backgroundMat, config, newPool)

            setPool(newPool)
            return;
        }

        render(element, backgroundMat, config, pool)
    }, [config, element, backgroundMat])

    const render = (element, backgroundMat, config, workerPool) => {
        setFramesRendering(frames => frames + 1)
        requestEmitter.render_element(element, backgroundMat, config, workerPool)
        .then(wasm_image => {
            setFramesRendering(frames => frames - 1)
            displayImage(wasm_image)  
        })
    }

    const colorToRGB = (color) => {
        return `rgb(${color.red}, ${color.green}, ${color.blue})`;
    }

    const displayImage = (image) => {
        let width = image.width;
        let height = image.height;
        let buffer = image.buffer;

        let canvas: any = canvasRef.current;

        let context = canvas.getContext("2d");

        for (let i = 0; i < height; ++i) {
            for (let j = 0; j < width; ++j) {
                let color = buffer[i][j];

                context.setFillColor(colorToRGB(color));
                context.fillRect(j, height - 1 - i, 1, 1);
            }
        }

        setCanvasImageURL(getCanvasURL);
    }

    const onMatChange = (mat) => {
        console.log("Updated material", mat)
        let elementClone = structuredClone(element);
        elementClone.material = mat;

        onElementUpdate(elementClone);
    }

    const onShapeChange = (shape) => {
        console.log("New shape", shape);
        let elementClone = structuredClone(element);
        elementClone.shape = shape;

        onElementUpdate(elementClone);

    }

    const getCanvasURL = (): string => {
        if (canvasRef.current === null) return;

        return canvasRef.current.toDataURL("image/png");
    }

    return (
        <div className="element-display">
            <div className="canvas-container">
                <Paper elevation={5}>
                    <canvas 
                        className="element-preview-canvas" 
                        width={750} 
                        height={750} 
                        ref={canvasRef} 
                    />
                </Paper>
            </div>
            <div className="element-display-config">
                <Button 
                    className="download-button" 
                    variant="outlined" 
                    color="primary" 
                    size="small"
                >
                    <a 
                        className="download-link"
                        href={canvasImageURL}
                        download={`${Math.round(Math.random() * 100000)}_render.png`}
                    >Download Image</a>
                </Button>
                <span className="rendering-status">
                    {framesRendering > 0 ? 
                        `Rendering: ${framesRendering} frame${framesRendering > 1 ? 's' : ''}` :
                        "Idle"
                    }
                </span>
                <Card className="config-container">
                    <h3>Ray Tracer</h3>
                    <CameraDisplay />
                    <QualityDisplay />
                    <h3>Element</h3>
                    <ShapeDisplay shape={shape} onShapeChange={onShapeChange} />
                    <MaterialDisplay material={material} onMatChange={onMatChange} />
                    <h3>Background</h3>
                    <MaterialDisplay material={backgroundMat} onMatChange={setBackgroundMat} />
                </Card>
            </div>
       </div>
    )
}

export { ElementDisplay }
