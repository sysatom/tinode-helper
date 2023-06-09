import { useRouter } from "next/router";
import { useState } from "react";
import UpdateIcon from "~/svg/update.svg";
import {actionFetcher} from "~/helpers/fetcher";
import { deleteStore, getStore, setStore } from "~/helpers/store";

const TokenRegistration = () => {
  const [token, setToken] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState("");
  const router = useRouter();

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setIsSubmitting(true);
    setError("");
    try {
      await setStore("access-url", token);
      await actionFetcher("info");
      router.push("/dashboard");
    } catch (error) {
      setError("Invalid access url " + error);
      deleteStore("access-url").then();
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <section className="flex flex-col w-full p-4">
      <form className="flex flex-col w-full mt-3" onSubmit={handleSubmit}>
        <label
          htmlFor="token"
          className="text-sm font-normal text-zinc-900 dark:text-zinc-50"
        >
          Tinode Bot Access Url
        </label>
        <input
          type="text"
          id="token"
          name="token"
          value={token}
          onChange={(e) => setToken(e.target.value)}
          className={`w-full mt-1 px-3 py-2 border font-normal border-zinc-200 dark:border-zinc-700 rounded-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 dark:focus-visible:ring-blue-400 focus-visible:border-transparent ${
            isSubmitting
              ? "opacity-40 cursor-not-allowed"
              : "opacity-100 cursor-auto"
          }`}
          placeholder="access url"
          disabled={isSubmitting}
        />
        <button
          type="submit"
          className={`w-full mt-2 py-2 px-3 border border-zinc-200 dark:border-zinc-700 rounded-md bg-zinc-900 dark:bg-zinc-50 text-zinc-50 dark:text-zinc-900 font-medium text-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-zinc-500 dark:focus-visible:ring-zinc-400 focus-visible:border-transparent ${
            isSubmitting
              ? "cursor-not-allowed"
              : "cursor-default hover:bg-blue-500 hover:text-zinc-50 hover:dark:bg-blue-600 hover:dark:text-zinc-50"
          }  duration-75`}
          disabled={isSubmitting}
        >
          {isSubmitting ? (
            <UpdateIcon className="animate-spin text-zinc-400 dark:text-zinc-500 mx-auto" />
          ) : (
            "Get Started"
          )}
        </button>
        {error && (
          <p className="text-sm font-normal text-red-500 mt-2">{error}</p>
        )}
      </form>

      <p className="text-sm text-zinc-700 dark:text-zinc-300 mt-4">
        Steps for creating a Access Url : Enter the command `access url` in helper bot.
      </p>
    </section>
  );
};

export default TokenRegistration;
