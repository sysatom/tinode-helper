import Image from "next/image";
import { useRouter } from "next/router";
import { Fragment, useEffect, useState } from "react";
import useSWR from "swr";
import Header from "~/components/common/Header";
import ProjectsDropdown from "~/components/ProjectsDropdown";
import GearIcon from "~/svg/gear.svg";
import {actionFetcher} from "~/helpers/fetcher";
import {IInfo} from "~/types";
import Link from "next/link";
import Bots from "~/components/Bots";
import {deleteStore, getStore} from "~/helpers/store";

const Dashboard = () => {
  const { data, error } = useSWR<IInfo, Error>("info", actionFetcher);
  const router = useRouter();
  const [selectedBot, setSelectedBot] = useState<string>("");
  const [id, setId] = useState<string>("");

  useEffect(()=> {
    if (
        error?.message === "No access url found" ||
        error?.message === "Not authenticated" ||
        error?.message === "Request error"
    ) {
      deleteStore("access-url").then();
      setTimeout(() => {
        router.push("/");
      }, 1000);
    }
  }, [error])

  const handleProjectChange = (id: string) => {
    setSelectedBot(id);
  };

  // if (!data) return;

  // get id
  getStore("access-url").then(url => {
    if (url) {
      let r = /\d{20,}/;
      setId((url as string).match(r)?.join('') || "-");
    }
  });

  return (
    <Fragment>
      <Header classes="justify-between">
        <div className="flex items-center">
          <Image
            src={`https://vercel.com/api/www/avatar/user?s=60`}
            width={32}
            height={32}
            alt="Avatar"
            className="rounded-full"
          />
          <section className="ml-2 flex flex-col">
            <p className="text-base font-medium text-zinc-900 dark:text-zinc-50">
              {data?.username}
            </p>
            <p className="text-xs font-normal text-zinc-500 dark:text-zinc-400">
              {id}
            </p>
          </section>
        </div>
        <div className="flex items-center">
          <ProjectsDropdown
            selectedProject={selectedBot}
            handleProjectChange={handleProjectChange}
          />
          <Link href="/settings">
            <button className="ml-2 bg-transparent hover:bg-zinc-200 hover:dark:bg-zinc-700 p-2 rounded">
              <GearIcon />
            </button>
          </Link>
        </div>
      </Header>
      <Bots selectedBot={selectedBot} />
    </Fragment>
  );
};

export default Dashboard;
