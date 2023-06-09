import { useTheme } from "next-themes";
import Link from "next/link";
import { useRouter } from "next/router";
import { Fragment, useEffect, useState } from "react";
import AlertDialog from "~/components/common/AlertDialog";
import Header from "~/components/common/Header";
import RadioGroup from "~/components/common/RadioGroup";
import Slider from "~/components/common/Slider";
import LeftArrow from "~/svg/arrow-left.svg";
import {deleteStore} from "~/helpers/store";

const THEMES = [
  {
    label: "System",
    value: "system",
    renderer: "System",
  },
  {
    label: "Light",
    value: "light",
    renderer: "Light",
  },
  {
    label: "Dark",
    value: "dark",
    renderer: "Dark",
  },
];

const NOTIFICATIONS = [
  {
    label: "Yes",
    value: "yes",
    renderer: "Yes",
  },
  {
    label: "No",
    value: "no",
    renderer: "No",
  },
];

const SettingsPage = () => {
  const [mounted, setMounted] = useState(false);
  const { theme, setTheme } = useTheme();
  const router = useRouter();
  const [showConfirmModal, setShowConfirmModal] = useState(false);
  const [notificationSetting, setNotificationSetting] = useState("no");
  const [frequency, setFrequency] = useState([10]);

  // When mounted on client, now we can show the UI
  useEffect(() => {
    setMounted(true);
    setNotificationSetting(
      window.localStorage.getItem("notifications") || "no"
    );
    setFrequency([parseInt(window.localStorage.getItem("frequency") || "15")]);
  }, []);

  if (!mounted) return null;

  const handleSetNotification = (value: string) => {
    setNotificationSetting(value);
    window.localStorage.setItem("notifications", value);
  };

  const handleFrequencyChange = (value: number[]) => {
    setFrequency(value);
    window.localStorage.setItem("frequency", value[0].toString());
  };

  const handleClickLogout = () => {
    setShowConfirmModal(true);
  };

  return (
    <Fragment>
      <Header classes="items-center">
        <Link href="/dashboard">
          <button className="mr-2 bg-transparent hover:bg-zinc-200 hover:dark:bg-zinc-700 p-2 rounded">
            <LeftArrow />
          </button>
        </Link>
        <h1 className="text-base font-medium text-zinc-900 dark:text-zinc-50">
          Settings
        </h1>
      </Header>
      <div className="p-4 flex flex-col h-full">
        <section className="flex items-center justify-between">
          <div className="flex flex-col">
            <p className="text-sm font-medium text-zinc-700 dark:text-zinc-200">
              Theme
            </p>
            <p className="text-xs font-normal text-zinc-500 dark:text-zinc-400">
              How would you like Helper to look?
            </p>
          </div>
          <RadioGroup
            label="Theme"
            items={THEMES}
            value={theme}
            handleChange={(val) => setTheme(val)}
          />
        </section>
        <section className="flex items-center justify-between mt-8">
          <div className="flex flex-col">
            <p className="text-sm font-medium text-zinc-700 dark:text-zinc-200">
              Send Notifications
            </p>
            <p className="text-xs font-normal text-zinc-500 dark:text-zinc-400">
              Send system messages when actions.
            </p>
          </div>
          <RadioGroup
            label="Notifications"
            items={NOTIFICATIONS}
            value={notificationSetting}
            handleChange={(val) => handleSetNotification(val)}
          />
        </section>
        <section className="flex items-center justify-between mt-8">
          <div className="flex flex-col">
            <p className="text-sm font-medium text-zinc-700 dark:text-zinc-200">
              Request Interval
            </p>
            <p className="text-xs font-normal text-zinc-500 dark:text-zinc-400">
              How frequently should Helper check for new action?
            </p>
          </div>
          <Slider
            max={60}
            min={1}
            step={1}
            label="Interval"
            value={frequency}
            handleChange={handleFrequencyChange}
          />
        </section>
        <section className="flex items-center justify-between mt-8">
          <div className="flex flex-col">
            <p className="text-sm font-medium text-zinc-700 dark:text-zinc-200">
              Logging
            </p>
            <p className="text-xs font-normal text-zinc-500 dark:text-zinc-400">
              View all logs.
            </p>
          </div>
          <Link href="/logs">
            <button className="mr-2 bg-transparent hover:bg-zinc-200 hover:dark:bg-zinc-700 p-2 rounded">
              view
            </button>
          </Link>
        </section>
        <section className="mt-auto flex justify-between items-center p-4 border border-red-400 rounded bg-red-200/30 dark:bg-red-800/20">
          <div className="flex flex-col">
            <p className="text-sm font-medium text-zinc-700 dark:text-zinc-200">
              Stop?
            </p>
            <p className="text-xs font-normal text-zinc-500 dark:text-zinc-400">
              Stop of Helper and erase saved access url.
            </p>
          </div>
          <AlertDialog
            handleTriggerClick={handleClickLogout}
            confirmLabel="Stop"
            show={showConfirmModal}
            title="Are you absolutely sure?"
            description="You will be stop and your access url will be deleted from local storage"
            triggerLabel="Stop"
            handleConfirm={() => {
              deleteStore("access-url").then();
              setTimeout(() => {
                router.push("/");
              }, 1000);
            }}
          />
        </section>
      </div>
    </Fragment>
  );
};

export default SettingsPage;
