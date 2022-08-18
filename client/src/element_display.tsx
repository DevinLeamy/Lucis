import { ReactJSXElement } from "@emotion/react/types/jsx-namespace";
import { Button, FormControl, Card, FormControlLabel, FormLabel, Paper, Radio, RadioGroup, Slider } from "@mui/material";
import React, { ReactNode, useEffect, useRef, useState } from "react"; 
import { RequestEmitter, WorkerPool } from "./wasm_loader";

import "./styles.css";

interface ElementDisplayProps {
    element: any,
    onElementUpdate: (element) => {} 
}

const METAL = "Metal"
const LAMBERTIAN = "Lambertian"
const DIELECTRIC = "Dielectric"

const TEXUTURE_DEFAULT = { SolidTexture: { color: { red: 0.2, green: 0.2, blue: 0.9 } } };

const LAMBERTIAN_DEFAULT = {
    Lambertian: {
        texture: TEXUTURE_DEFAULT
    }
}

const METAL_DEFAULT = {
    Metal: {
        fuzz: 0.0,
        texture: TEXUTURE_DEFAULT
    }
}

const DIELECTRIC_DEFAULT = {
    Dielectric: {
        ref_index: 1.5
    }
}

let threadCount = 3; // navigator.hardwareConcurrency;

let objName = (obj) => {
    return Object.keys(obj)[0]
}

const ElementDisplay = ({ element, onElementUpdate }: ElementDisplayProps) => {
    const [pool, setPool] = useState(undefined); // new WorkerPool(threadCount));

    let requestEmitter = new RequestEmitter();

    const canvasRef = useRef();
    const { material, shape } = element;

    useEffect(() => {
        setPool(new WorkerPool(threadCount));
    }, [])

    console.log("Displayed element", element)

    const renderElement = (element) => {
        requestEmitter.render_element(element, pool)
            .then(wasm_image => {
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
    }

    const onMatChange = (mat) => {
        console.log("Updated material", mat)
        let elementClone = structuredClone(element);
        elementClone.material = mat;

        onElementUpdate(elementClone);
        renderElement(elementClone);
    }

    return (
        <div>
            <div className="canvas-container">
                <Paper elevation={5}>
                    <canvas className="element-preview-canvas" width={600} height={600} ref={canvasRef} />
                </Paper>
            </div>
            <Button 
                className="render-button" 
                variant="outlined" 
                color="primary" 
                onClick={(e) => renderElement(element)}
            >
                Render
            </Button>
            <Card>
                <MaterialDisplay material={material} onMatChange={onMatChange} />
            </Card>
        </div>
    )
}

const MaterialDisplay = ({ material, onMatChange }) => {
    let matType = objName(material);

    console.log("Mat", material)

    const displayOptions = () => {
        switch (matType) {
            case METAL: return displayMetal(material.Metal);
            case LAMBERTIAN: return displayLambertian(material.Lambertian);
            case DIELECTRIC: return displayDielectric(material.Dielectric);
        }

        return <>Error</> 
    }
    
    
    const displayMetal = ({fuzz, texture}) : ReactJSXElement  => {

        let updateFuzz = (_e, value: number, __e) => {
            if (value === fuzz) return;

            let matClone = structuredClone(material)
            matClone.Metal.fuzz = value;

            onMatChange(matClone);
        }

        return (
            <div>
                <div className="config-slider-label">{`Fuzz (${fuzz})`}</div>
                <Slider 
                    min={0.0}
                    max={1.0}
                    step={0.05}
                    value={fuzz}
                    aria-label="Default" 
                    valueLabelDisplay="auto" 
                    onChange={updateFuzz}
                />
            </div>
        )
    }
    
    const displayLambertian = (m) : ReactJSXElement => {
        return <>Lambertian</> 
    }

    const displayDielectric = ({ ref_index }) : ReactJSXElement => {
        let updateRefIndex = (_e, value: number, __e) => {
            if (value === ref_index) return;

            let matClone = structuredClone(material)
            matClone.Dielectric.ref_index = value;

            onMatChange(matClone);
        }

        const marks = [
            { value: 1, label: 'Vaccum', },
            { value: 1.3, label: 'Water', },
            { value: 1.5, label: 'Glass', },
            { value: 2.4, label: 'Diamond', },
        ];

        // TODO: Add 
        return (
            <div>
                <div className="config-slider-label">{`Refractive Index (${ref_index})`}</div>
                <Slider 
                    min={1.0}
                    max={3.0}
                    step={0.05}
                    value={ref_index}
                    aria-label="Default" 
                    valueLabelDisplay="auto" 
                    onChange={updateRefIndex}
                    marks={marks}
                />
            </div>
        )
    }

    const MatChoice = ({ name }) : ReactJSXElement => {
        let checked = name === matType;

        return (
            <FormControlLabel 
                value={name}
                control={<Radio />}
                label={name}
                checked={checked}
            />
        )
    }

    const onMatTypeChange = (_e, newMatType) => {
        if (newMatType === matType) return;

        console.log()

        switch (newMatType) {
            case METAL: { onMatChange(METAL_DEFAULT); break; }
            case LAMBERTIAN: { onMatChange(LAMBERTIAN_DEFAULT); break; }
            case DIELECTRIC: { onMatChange(DIELECTRIC_DEFAULT); break; }
        }
    }

    return (
        <div className="material-display-container">
            <div className="material-types-container">
                <FormLabel id="material-button-group">Material</FormLabel>
                <RadioGroup
                    row
                    aria-labelledby="demo-radio-buttons-group-label"
                    value={matType}
                    name="material-button-group"
                    onChange={onMatTypeChange}
                >
                    <MatChoice name={LAMBERTIAN} />
                    <MatChoice name={DIELECTRIC} />
                    <MatChoice name={METAL} />
                </RadioGroup>
            </div>
            <div className="material-options-container">
                {displayOptions()}
            </div>
        </div>
    );
}

export { ElementDisplay }
