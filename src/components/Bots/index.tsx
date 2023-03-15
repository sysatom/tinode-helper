import {useEffect} from "react";
import useSWR from "swr";
import {actionFetcher} from "~/helpers/fetcher";
import {IBot} from "~/types";
import * as Switch from '@radix-ui/react-switch';

interface IBotsProps {
    selectedBot: string;
}

const Bots = ({selectedBot}: IBotsProps) => {
    const {data, error} = useSWR<IBot, Error>("bots", actionFetcher)

    useEffect(() => {
       if (selectedBot != "") {
           console.log(selectedBot)
       }
    });

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
                        >
                            <Switch.Thumb className="block w-[21px] h-[21px] bg-white rounded-full shadow-[0_2px_2px] shadow-blackA7 transition-transform duration-100 translate-x-0.5 will-change-transform data-[state=checked]:translate-x-[19px]" />
                        </Switch.Root>
                        <div className="w-[320px] text-sm font-medium text-zinc-700 dark:text-zinc-200 pl-4">agent, command</div>
                    </div>
                ))
            }</div>) : (<div>{selectedBot}</div>)
        }
        </div>
    );
}

export default Bots
