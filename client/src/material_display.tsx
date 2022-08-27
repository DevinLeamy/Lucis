import { ReactJSXElement } from "@emotion/react/types/jsx-namespace";
import { Button, FormControl, Card, FormControlLabel, FormLabel, Paper, Radio, RadioGroup, Slider } from "@mui/material";
import React from "react"; 
import { TEXTURE_DEFAULT, objName } from "./utils";
import { TextureDisplay } from "./texture_display";

import "./styles.css";

const METAL = "Metal"
const LAMBERTIAN = "Lambertian"
const DIELECTRIC = "Dielectric"

const LAMBERTIAN_DEFAULT = { Lambertian: { texture: TEXTURE_DEFAULT } }
const METAL_DEFAULT = { Metal: { fuzz: 0.0, texture: TEXTURE_DEFAULT } }
const DIELECTRIC_DEFAULT = { Dielectric: { ref_index: 1.5 } }


const MaterialDisplay = ({ material, onMatChange }) => {
    let matType = objName(material);

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
                    size={"small"}
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
                    size={"small"}
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

export { MaterialDisplay }
