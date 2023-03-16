import {useEffect, useState} from "react";
import useSWR from "swr";
import {actionFetcher} from "~/helpers/fetcher";
import {IBot} from "~/types";
import * as Switch from '@radix-ui/react-switch';

interface IBotsProps {
    selectedBot: string;
}

const Bots = ({selectedBot}: IBotsProps) => {
    let [botSwitch, setBotSwitch] = useState<Map<String, boolean>>(() => {
        const value = localStorage.getItem("bot-switch")
        if (value) {
            let result = JSON.parse(value)
            if (result) {
                let m = result as Array<any>
                if (m != undefined) {
                    const newMap = new Map<String, boolean>()
                    m.forEach(([key, value]) => {
                        newMap.set(key, value);
                    });
                    return newMap
                }
            }
        }
        return new Map<string, any>()
    })
    const {data, error} = useSWR<IBot, Error>("bots", actionFetcher)

    let [help, setHelp] = useState<{ [key: string]: Array<String> }>({})
    useEffect(() => {
       if (selectedBot != "") {
           actionFetcher("help", selectedBot).then(r => setHelp(r))
       }
    });

    const handleUpdateValue = (key:string, newValue:boolean) => {
        const newMap = new Map<String, boolean>(Array.from(botSwitch).map(([k, v]) => (k === key ? [k, newValue] : [k, v])));
        newMap.set(key, newValue)
        localStorage.setItem("bot-switch", JSON.stringify(Array.from(newMap)))
        setBotSwitch(newMap);
    };

    return (
        <div>{
            selectedBot == "" ? (<div className="flex flex-col h-full p-4">{
                data?.bots.map(b => (
                    <div key={b.id} className="flex items-center p-2" style={{ display: 'flex', alignItems: 'center' }}>
                        <label className="text-sm font-medium text-zinc-700 dark:text-zinc-200 leading-none pr-[15px]" htmlFor="airplane-mode">
                            {b.name}
                        </label>
                        <Switch.Root
                            className="w-[42px] h-[25px] bg-blackA9 rounded-full relative shadow-[0_2px_10px] shadow-blackA7 focus:shadow-[0_0_0_2px] focus:shadow-black data-[state=checked]:bg-black outline-none cursor-default"
                            id="airplane-mode"
                            defaultChecked={botSwitch.get(b.id)}
                            onCheckedChange={(checked: boolean) => {handleUpdateValue(b.id, checked)}}
                        >
                            <Switch.Thumb className="block w-[21px] h-[21px] bg-white rounded-full shadow-[0_2px_2px] shadow-blackA7 transition-transform duration-100 translate-x-0.5 will-change-transform data-[state=checked]:translate-x-[19px]" />
                        </Switch.Root>
                    </div>
                ))
            }</div>) : (<div style={{height: "500px", overflow: "auto"}}>
                <h2 className="text-center text-xl mt-3 mb-3">{selectedBot}</h2>
                <table style={{width: "90%", margin: "auto"}}>
                    <tr>
                        <th>Type</th>
                        <th>Help</th>
                    </tr>
                    {
                        Object.keys(help).map((key) => {
                            return help[key].map(v => <tr>
                                <td>{key}</td>
                                <td>{v}</td>
                            </tr>)
                        })
                    }
                </table>
            </div>)
        }
        </div>
    );
}

export default Bots
