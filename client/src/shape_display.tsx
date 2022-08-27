import { ReactJSXElement } from "@emotion/react/types/jsx-namespace";
import { Button, FormControl, Card, FormControlLabel, FormLabel, Paper, Radio, RadioGroup, Slider } from "@mui/material";
import React from "react"; 
import { objName } from "./utils";
import { RequestEmitter } from "./wasm_loader";

const SPHERE = "Sphere";
const BOX    = "Box";

const SPHERE_DEFAULT = { Sphere: { radius: 0.5, center: { x: 0.0, y: 0.5, z: 0.0 } } }

const ShapeDisplay = ({ shape, onShapeChange }) => {
    let shapeType = objName(shape);

    const displayOptions = () => {
        switch (shapeType) {
            case SPHERE: return displaySphere(shape.Sphere);
            case BOX:    return displayBox(shape.Box);
        }

        return <>Invalid shape</> 
    }

    const displayBox = ({ min, max, _sides }) => {
        return (
            <div>Box Options</div>
        )
    }
    
    
    const displaySphere= ({radius, _center}) : ReactJSXElement  => {

        let updateRadius = (_e, newRadius: number, __e) => {
            if (radius === newRadius) return;

            let sphereClone = structuredClone(shape);
            sphereClone.Sphere.radius = newRadius;

            onShapeChange(sphereClone);
        }

        return (
            <div>
                <div className="config-slider-label">{`Radius (${radius})`}</div>
                <Slider 
                    min={0.05}
                    max={1.0}
                    step={0.05}
                    value={radius}
                    aria-label="Default" 
                    size={"small"}
                    valueLabelDisplay="auto" 
                    onChange={updateRadius}
                />
            </div>
        )
    }
    
    const ShapeChoice = ({ name }) : ReactJSXElement => {
        let checked = name === shapeType;

        return (
            <FormControlLabel 
                value={name}
                control={<Radio />}
                label={name}
                checked={checked}
            />
        )
    }

    const onShapeTypeChange = (_e, newMatType) => {
        if (newMatType === shapeType) return;

        switch (newMatType) {
            case SPHERE: { onShapeChange(SPHERE_DEFAULT); break; }
            case BOX:    { onShapeChange(new RequestEmitter().get_default_box());    break; }
        }
    }

    return (
        <div className="shape-display-container">
            <div className="shape-types-container">
                <FormLabel id="shape-button-group">Shape</FormLabel>
                <RadioGroup
                    row
                    value={shapeType}
                    name="shape-button-group"
                    onChange={onShapeTypeChange}
                >
                    <ShapeChoice name={SPHERE} />
                    <ShapeChoice name={BOX} />
                </RadioGroup>
            </div>
            <div className="shape-options-container">
                {displayOptions()}
            </div>
        </div>
    );
}

export { ShapeDisplay };
