import { useRouter } from "next/router";
import { Fragment, useEffect } from "react";
import Header from "~/components/common/Header";
import TokenRegistration from "~/components/TokenRegistration";
import {deleteStore, getStore} from "~/helpers/store";
import {actionFetcher} from "~/helpers/fetcher";

function Index() {
  const router = useRouter();

  useEffect(() => {
    getStore("access-url").then(url => {
      if (url) {
        actionFetcher("info").then(r => {
          router.push("/dashboard");
        }).catch(err => {
          console.log("index info", err)
          deleteStore("access-url").then();
        })
      }
    })
  })

  return (
    <Fragment>
      <Header classes="flex-col">
        <h1 className="text-2xl font-bold text-zinc-900 dark:text-zinc-50 mb-1">
          Helper
        </h1>
        <p className="text-sm font-normal text-zinc-700 dark:text-zinc-200">
          Quickly displays your bots right in the menu bar
        </p>
      </Header>
      <TokenRegistration />
    </Fragment>
  );
}

export default Index;
