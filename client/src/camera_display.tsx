import { ReactJSXElement } from "@emotion/react/types/jsx-namespace";
import { FormControlLabel, FormLabel, Radio, RadioGroup } from "@mui/material";
import React, { useContext, useState, useEffect } from "react"; 
import { ConfigContext } from "./contexts/config";


export const CAMERA_OPTIONS = {
    "DEFAULT_VIEW": {
        origin:  { x: 0.0, y: 2.0, z: 4.0 },
        lookAt: { x: 0.0, y: 0.2, z: 0.5 }
    },
    "TOP_VIEW": {
        origin:  { x: 0.0, y: 4.0, z: 0.01 },
        lookAt: { x: 0.0, y: 0.0, z: 0.0 }
    },
    "LEFT_SIDE_VIEW": {
        origin:  { x: 4.0, y: 3.5, z: 4.0 },
        lookAt: { x: 0.0, y: 0.0, z: 0.0 }
    },
    "RIGHT_SIDE_VIEW": {
        origin:  { x: -4.0, y: 3.5, z: 4.0 },
        lookAt: { x: 0.0, y: 0.0, z: 0.0 }
    },
}

export type CameraConfig = {
    origin: { x: number, y: number, z: number },
    lookAt: { x: number, y: number, z: number },
}

const CameraDisplay = () => {
    const { onSetConfig } = useContext(ConfigContext)
    const [cameraType, setCameraType] = useState("DEFAULT_VIEW")

    useEffect(() => {
        onSetConfig(CAMERA_OPTIONS[cameraType])
    }, [cameraType])

    const onCameraTypeChange = (_, newCameraType) => {
        if (cameraType === newCameraType) return;

        setCameraType(newCameraType)
    }

    const CameraChoice = ({ name }) : ReactJSXElement => {
        let checked = name === cameraType;
        let formatName = (rawName: string) => {
            rawName = rawName.replace(/_/g, ' ').toLowerCase()
            rawName = rawName[0].toUpperCase() + rawName.slice(1);

            return rawName
        }

        return (
            <FormControlLabel 
                value={name}
                control={<Radio size={"small"}/>}
                label={formatName(name)}
                checked={checked}
            />
        )
    }

    return (
        <div className="camera-display-container">
            <div className="camera-types-container">
                <FormLabel id="camera-button-group">Perspective</FormLabel>
                <RadioGroup
                    row
                    value={cameraType}
                    name="camera-button-group"
                    onChange={onCameraTypeChange}
                >
                    <CameraChoice name={"DEFAULT_VIEW"} />
                    <CameraChoice name={"TOP_VIEW"} />
                    <CameraChoice name={"LEFT_SIDE_VIEW"} />
                    <CameraChoice name={"RIGHT_SIDE_VIEW"} />
                </RadioGroup>
            </div>
        </div>
    );
}

export { CameraDisplay };
