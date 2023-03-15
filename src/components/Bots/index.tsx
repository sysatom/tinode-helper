import {useEffect} from "react";
import useSWR from "swr";
import {actionFetcher} from "~/helpers/fetcher";
import {IBot} from "~/types";

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
            selectedBot == "" ? (<div>{
                data?.bots.map(b => (<div>{b.id}</div>))
            }</div>) : (<div>{selectedBot}</div>)
        }</div>
    );
}

export default Bots
