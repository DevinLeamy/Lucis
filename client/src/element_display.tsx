import { Button } from "@mui/material";
import React, {} from "react"; 

import { RequestEmitter } from "./wasm_loader";

interface ElementDisplayProps {
    wasm: any,
}

const ElementDisplay = (props: ElementDisplayProps) => {
    const test = (_e: any) => {
        let requestEmitter = new RequestEmitter();
        let element = requestEmitter.get_element();

        console.log(element);
    }

    return (
        <div>
            Element display
            <Button onClick={test}>
                Click Me
            </Button>
        </div>
    )
}

export { ElementDisplay }
