import { useEffect, useRef } from "react";
import useSWRInfinite from "swr/infinite";
import {actionFetcher} from "~/helpers/fetcher";
import { useIntersectionObserver } from "~/hooks";
import {IBot} from "~/types";

import Dropdown from "./common/Dropdown";

interface IProjectDropdownProps {
  handleProjectChange: (id: string) => void;
  selectedProject: string;
}

const getKey = (pageIndex: number) => {
  return `bots`;
};

const ProjectsDropdown = ({
  handleProjectChange,
  selectedProject,
}: IProjectDropdownProps) => {
  const { data, error, isLoading, isValidating, setSize, size } =
    useSWRInfinite<IBot>(getKey, actionFetcher, {
      revalidateOnFocus: false,
    });

  if (error) {
    return <p>Oops!</p>;
  }

  const availableProjects = data
    ?.map((page) =>
      page.bots.map((bot) => ({
        label: bot.name,
        id: bot.id,
      }))
    )
    .flat();

  availableProjects?.splice(0, 0, {
    label: "All Bots",
    id: "",
  });

  const scrollRef = useRef<HTMLDivElement | null>(null);
  const entry = useIntersectionObserver(scrollRef, {});

  const isVisible = !!entry?.isIntersecting;

  useEffect(() => {
    if (isVisible && !isLoading && !isValidating) {
      setSize(size + 1);
    }
  }, [isVisible]);

  return (
    <Dropdown
      triggerLabel={
        availableProjects?.find((project) => project.id === selectedProject)
          ?.label
      }
      items={availableProjects}
      selectedId={selectedProject}
      handleOnClick={handleProjectChange}
      scrollRef={scrollRef}
      isLoading={isLoading}
    />
  );
};

export default ProjectsDropdown;
