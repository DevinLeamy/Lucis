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

const ElementDisplay = ({ element, onElementUpdate }: ElementDisplayProps) => {
    const { config } = useContext(ConfigContext)
    const [pool, setPool] = useState<typeof WorkerPool>(undefined); 
    const [canvasImageURL, setCanvasImageURL] = useState<string>(undefined);

    console.log("Element", element)

    let requestEmitter = new RequestEmitter();

    const canvasRef = useRef<HTMLCanvasElement>(null);

    const { material, shape } = element;

    useEffect(() => {
        if (pool === undefined) {
            let newPool = new WorkerPool(threadCount);

            requestEmitter.render_element(element, config, newPool)
                .then(wasm_image => displayImage(wasm_image))

            setPool(newPool)
            return;
        }

        requestEmitter.render_element(element, config, pool)
            .then(wasm_image => displayImage(wasm_image))
    }, [config, element])

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
        <div>
            <div className="canvas-container">
                <Paper elevation={5}>
                    <canvas 
                        className="element-preview-canvas" 
                        width={750} 
                        height={450} 
                        ref={canvasRef} 
                    />
                </Paper>
            </div>
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
            <Card className="config-container">
                <CameraDisplay />
                <QualityDisplay />
                <ShapeDisplay shape={shape} onShapeChange={onShapeChange} />
                <MaterialDisplay material={material} onMatChange={onMatChange} />
            </Card>
        </div>
    )
}

export { ElementDisplay }
