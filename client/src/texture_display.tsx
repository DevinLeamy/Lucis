import { ReactJSXElement } from "@emotion/react/types/jsx-namespace";
import { FormControlLabel, FormLabel, Paper, Radio, RadioGroup, Slider } from "@mui/material";
import React from "react"; 
import { CompactPicker } from "react-color";
import { TEXTURE_DEFAULT, objName } from "./utils";

import "./styles.css";


const SOLID_TEXTURE = "SolidTexture";
const CHECKERED_TEXTURE = "CheckeredTexture";
const PERLIN_TEXTURE = "PerlinTexture";

const SOLID_TEXTURE_DEFAULT = structuredClone(TEXTURE_DEFAULT);
const CHECKERED_TEXTURE_DEFAULT = { CheckeredTexture: {
    odd: { red: 1.0, green: 1.0, blue: 1.0 }, 
    even: { red: 0.2, green: 0.2, blue: 0.2 }, 
}};
const PERLIN_TEXTURE_DEFAULT = { PerlinTexture: { 
    scale: 1.0 
}};

const TextureDisplay = ({ texture, onTextureChange }) => {
    let textureType = objName(texture);

    console.log(texture);

    const displayOptions = () : ReactJSXElement => {
        switch (textureType) {
            case SOLID_TEXTURE:     return displaySolid(texture.SolidTexture);
            case CHECKERED_TEXTURE: return displayCheckered(texture.CheckeredTexture);
            case PERLIN_TEXTURE:    return displayPerlin(texture.PerlinTexture); 
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

    const displayPerlin = ({ scale }) : ReactJSXElement => {
        const onScaleChange = (_e, newScale) => {
            let textureClone = structuredClone(texture); 
            textureClone.PerlinTexture.scale = newScale;
            onTextureChange(textureClone); 
        }

        return (
            <div>
                <div className="config-slider-label">{`Scale (${scale})`}</div>
                <Slider 
                    min={0.05}
                    max={1.0}
                    step={0.05}
                    value={scale}
                    valueLabelDisplay="auto" 
                    onChange={onScaleChange}
                />
            </div> 
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
            case PERLIN_TEXTURE:    { onTextureChange(PERLIN_TEXTURE_DEFAULT);    break; }
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
                    <TextureChoice textureName={PERLIN_TEXTURE} />
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

export { TextureDisplay }
