import { ReactJSXElement } from "@emotion/react/types/jsx-namespace";
import { Button, FormControl, Card, FormControlLabel, FormLabel, Paper, Radio, RadioGroup, Slider } from "@mui/material";
import React, { ReactNode, useEffect, useRef, useState } from "react"; 
import { RequestEmitter, WorkerPool } from "./wasm_loader";
import { SliderPicker, CompactPicker, CirclePicker } from "react-color";

import "./styles.css";

interface ElementDisplayProps {
    element: any,
    onElementUpdate: (element) => {} 
}

const METAL = "Metal"
const LAMBERTIAN = "Lambertian"
const DIELECTRIC = "Dielectric"

const TEXTURE_DEFAULT = { SolidTexture: { color: { red: 0.2, green: 0.2, blue: 0.9 } } };
const SOLID_TEXTURE_DEFAULT = structuredClone(TEXTURE_DEFAULT);
const CHECKERED_TEXTURE_DEFAULT = { CheckeredTexture: {
    odd: { red: 1.0, green: 1.0, blue: 1.0 }, 
    even: { red: 0.2, green: 0.2, blue: 0.2 }, 
}}

const LAMBERTIAN_DEFAULT = { Lambertian: { texture: TEXTURE_DEFAULT } }
const METAL_DEFAULT = { Metal: { fuzz: 0.0, texture: TEXTURE_DEFAULT } }
const DIELECTRIC_DEFAULT = { Dielectric: { ref_index: 1.5 } }

let threadCount = 5; // navigator.hardwareConcurrency;

let objName = (obj) => {
    return Object.keys(obj)[0]
}

const ElementDisplay = ({ element, onElementUpdate }: ElementDisplayProps) => {
    const [pool, setPool] = useState<typeof WorkerPool>(undefined); 
    const [canvasImageURL, setCanvasImageURL] = useState<string>(undefined);

    let requestEmitter = new RequestEmitter();

    const canvasRef = useRef<HTMLCanvasElement>(null);

    const { material, shape } = element;

    useEffect(() => {
        let pool = new WorkerPool(threadCount);

        requestEmitter.render_element(element, pool)
            .then(wasm_image => displayImage(wasm_image))

        setPool(pool);
    }, [])

    // console.log("Displayed element", element)

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

        setCanvasImageURL(getCanvasURL);
    }

    const onMatChange = (mat) => {
        console.log("Updated material", mat)
        let elementClone = structuredClone(element);
        elementClone.material = mat;

        onElementUpdate(elementClone);
        renderElement(elementClone);
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
                        height={600} 
                        ref={canvasRef} 
                    />
                </Paper>
            </div>
            <Button 
                className="render-button" 
                variant="outlined" 
                color="primary" 
                size="small"
                onClick={(_e) => renderElement(element)}
            >
                Render
            </Button>
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
            <Card>
                <MaterialDisplay material={material} onMatChange={onMatChange} />
            </Card>
        </div>
    )
}

const MaterialDisplay = ({ material, onMatChange }) => {
    let matType = objName(material);

    // console.log("Mat", material)

    const onTextureChange = (newTexture) => {
        console.log("Updated texture", newTexture)
        let materialClone = structuredClone(material);

        switch (matType) {
            case METAL: {
                materialClone.Metal.texture = newTexture;
                onMatChange(materialClone);
                break;
            }
            case LAMBERTIAN: {
                materialClone.Lambertian.texture = newTexture;
                onMatChange(materialClone);
                break;
            }
        } 
    }

    const displayOptions = () => {
        switch (matType) {
            case METAL: return displayMetal(material.Metal);
            case LAMBERTIAN: return displayLambertian(material.Lambertian);
            case DIELECTRIC: return displayDielectric(material.Dielectric);
        }

        return <>Invalid material</> 
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
                <TextureDisplay texture={texture} onTextureChange={onTextureChange} />
            </div>
        )
    }
    
    const displayLambertian = ({ texture }) : ReactJSXElement => {
        return (
            <TextureDisplay texture={texture} onTextureChange={onTextureChange} />
        )
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

        switch (newMatType) {
            case METAL:      { onMatChange(METAL_DEFAULT);      break; }
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

const SOLID_TEXTURE = "SolidTexture";
const CHECKERED_TEXTURE = "CheckeredTexture";

const TextureDisplay = ({ texture, onTextureChange }) => {
    let textureType = objName(texture);

    // console.log("Texture", texture);

    const displayOptions = () : ReactJSXElement => {
        switch (textureType) {
            case SOLID_TEXTURE: return displaySolid(texture.SolidTexture);
            case CHECKERED_TEXTURE: return displayCheckered(texture.CheckeredTexture);
        }

        return <>Invalid texture</>
    }

    const TextureChoice = ({ textureName }) : ReactJSXElement => {
        let checked = textureName === textureType;

        return (
            <FormControlLabel 
                value={textureName}
                control={<Radio />}
                label={textureName}
                checked={checked}
            />
        )
    }

    const displaySolid = ({ color }) : ReactJSXElement => {
        const onColorChange = (newColor) => {
            let textureClone = structuredClone(texture); 
            textureClone.SolidTexture.color = newColor;
            onTextureChange(textureClone); 
        }

        return (
            <div className="color-picker-container">
                <ColorPicker 
                    color={color}
                    onColorChange={onColorChange} 
                />
            </div>
        )
    }

    const displayCheckered = ({ even, odd }) : ReactJSXElement => {
        const updateEvenColor = (newColor) => {
            let textureClone = structuredClone(texture); 
            textureClone.CheckeredTexture.even = newColor;
            onTextureChange(textureClone);
        }

        const updateOddColor = (newColor) => {
            let textureClone = structuredClone(texture); 
            textureClone.CheckeredTexture.odd = newColor;
            onTextureChange(textureClone);
        }

        // console.log(even, odd)

        return (
            <div style={{display: "flex"}}>
                <div style={{marginRight: "10px" }}>
                    {" "}Even
                    <ColorPicker 
                        color={even}
                        onColorChange={updateEvenColor} 
                    />
                </div>
                <div>
                    {" "}Odd
                    <ColorPicker 
                        color={odd}
                        onColorChange={updateOddColor}
                    />
                </div>
           </div>
        )
    }

    const onTextureTypeChange = (_e, newTextureType: string) => {
        if (newTextureType === textureType) return;

        switch (newTextureType) {
            case SOLID_TEXTURE:     { onTextureChange(SOLID_TEXTURE_DEFAULT);     break; }
            case CHECKERED_TEXTURE: { onTextureChange(CHECKERED_TEXTURE_DEFAULT); break; }
        }
    }

    return (
        <div className="texture-display-container">
            <div className="texture-types-container">
                <FormLabel id="texture-button-group">Texture</FormLabel>
                <RadioGroup
                    row
                    aria-labelledby="demo-radio-buttons-group-label"
                    value={textureType}
                    name="material-button-group"
                    onChange={onTextureTypeChange}
                >
                    <TextureChoice textureName={SOLID_TEXTURE} />
                    <TextureChoice textureName={CHECKERED_TEXTURE} />
                </RadioGroup>
            </div>
            <div className="texture-options-container">
                {displayOptions()}
            </div>
        </div>
    )
}

const ColorPicker = ({ color, onColorChange }) => {
    const toPickerRGB = (rustRGB) => {
        return {
            r: rustRGB.red * 255,
            g: rustRGB.green * 255,
            b: rustRGB.blue * 255,
        }
    }

    const fromPickerRGB = (pickerRGB) => {
        return {
            red: pickerRGB.rgb.r / 255,
            green: pickerRGB.rgb.g / 255,
            blue: pickerRGB.rgb.b / 255,
        }
   }

    return (
        <div className="color-picker-container">
            <CompactPicker
                color={toPickerRGB(color)} 
                onChangeComplete={(newColor, _event) => {
                    onColorChange(fromPickerRGB(newColor));   
                }}
            />
        </div>
    )
}


export { ElementDisplay }
