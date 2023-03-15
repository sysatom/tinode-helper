interface IBotsProps {
    selectedBot: string;
}

const Bots = ({selectedBot}: IBotsProps) => {
    return (
        <div>bots {selectedBot}</div>
    );
}

export default Bots
