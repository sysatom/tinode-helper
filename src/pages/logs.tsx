import { Fragment } from "react";
import Header from "~/components/common/Header";
import Link from "next/link";
import LeftArrow from "~/svg/arrow-left.svg";

const Logs = () => {
    return (
        <Fragment>
            <Header classes="items-center">
                <Link href="/settings">
                    <button className="mr-2 bg-transparent hover:bg-zinc-200 hover:dark:bg-zinc-700 p-2 rounded">
                        <LeftArrow />
                    </button>
                </Link>
                <h1 className="text-base font-medium text-zinc-900 dark:text-zinc-50">
                    Logging
                </h1>
            </Header>
            <div>
                all logs
            </div>
        </Fragment>
    );
};

export default Logs;
