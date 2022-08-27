import React, { useContext, useState, Dispatch } from "react"

type Vec3 = {
    x: number,
    y: number,
    z: number
}

const defaultVec3: Vec3 = {
    x: 0, y: 0, z: 0
}

interface ConfigContextI {
    onSetConfig: (newConfig) => void, 
    config: any 
}

export type ClientConfig = {
    origin: Vec3,
    look_at: Vec3,
    max_bounce_depth: number,
    samples: number,
}

export const ConfigContext = React.createContext({} as ConfigContextI)

export const ConfigContextProvider: React.FC<{children}> = ({children}) => {
    const [origin, setOrigin] = useState<Vec3>(defaultVec3)
    const [lookAt, setLookAt] = useState<Vec3>(defaultVec3)
    const [samples, setSamples] = useState<number>(10)
    const [bounceDepth, setBounceDepth] = useState<number>(3)

    const config = {
        origin: origin ?? defaultVec3,
        look_at: lookAt ?? defaultVec3,
        max_bounce_depth: bounceDepth ?? 3,
        samples: samples ?? 10,
   } 

    const onSetConfig = (newConfig) => {
        if (newConfig.origin) setOrigin(newConfig.origin)
        if (newConfig.lookAt) setLookAt(newConfig.lookAt)
        if (newConfig.samples) setSamples(newConfig.samples)
        if (newConfig.bounceDepth) setBounceDepth(newConfig.bounceDepth)
    }


    return (
        <ConfigContext.Provider
            value={{
                onSetConfig,
                config
            }}
        >
            {children}
        </ConfigContext.Provider>
    )
}
