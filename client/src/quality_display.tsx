import { ReactJSXElement } from "@emotion/react/types/jsx-namespace";
import { FormControlLabel, FormLabel, Radio, RadioGroup } from "@mui/material";
import React, { useContext, useState } from "react"; 
import { ConfigContext } from "./contexts/config";


export const QUALITY_OPTIONS = {
    "DEFAULT": {
        samples: 20,
        bounceDepth: 50
    },
    "HIGH": {
        samples: 100,
        bounceDepth: 50
    },
    "LOW": {
        samples: 5,
        bounceDepth: 50
    },
    "VERY_HIGH": {
        samples: 300,
        bounceDepth: 100
    }
}

const QualityDisplay = () => {
    const { onSetConfig } = useContext(ConfigContext)
    const [quality, setQuality] = useState("DEFAULT")

    const onCameraTypeChange = (_, newQuality) => {
        if (quality === newQuality) return;

        onSetConfig(QUALITY_OPTIONS[newQuality]);
        setQuality(newQuality)
    }

    const QualityChoice = ({ name }) : ReactJSXElement => {
        let checked = name === quality;
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
                <FormLabel id="camera-button-group">Render Quality</FormLabel>
                <RadioGroup
                    row
                    value={quality}
                    name="camera-button-group"
                    onChange={onCameraTypeChange}
                >
                    <QualityChoice name={"LOW"} />
                    <QualityChoice name={"DEFAULT"} />
                    <QualityChoice name={"HIGH"} />
                    <QualityChoice name={"VERY_HIGH"} />
                </RadioGroup>
            </div>
        </div>
    );
}

export { QualityDisplay };
