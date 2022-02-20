import React from "react";

export const RoadmapSection = ({ date, title, desc, type }) => {
  return (
    <div className="flex w-96 text-left relative mb-10">
      <div
        className={`w-10 h-10 absolute -left-[58px] top-4 border-8 border-main ${
          type === "past"
            ? "bg-violet-700"
            : type === "soon"
            ? "bg-orange-600"
            : "bg-gray-600"
        } rounded-full`}
      />
      <div
        className={`w-0 h-0 top-7 absolute -left-2 border-t-8 border-b-8 border-r-8 border-t-transparent border-b-transparent ${
          type === "past"
            ? "border-r-violet-700"
            : type === "soon"
            ? "border-r-orange-600"
            : "border-r-gray-600"
        }`}
      />
      <div
        className={`timeline-body border-4 rounded-md py-5 px-3  ${
          type === "past"
            ? "border-violet-700"
            : type === "soon"
            ? "border-orange-600"
            : "border-gray-600"
        }`}
      >
        <div
          className={`px-3 py-1 w-max rounded-md text-sm mb-3 ${
            type === "past"
              ? "bg-violet-900"
              : type === "soon"
              ? "bg-orange-800"
              : "bg-gray-800"
          }`}
        >
          {date}
        </div>
        <h4 className="text-lg font-semibold mb-1">{title}</h4>
        <p className="italic text-sm">{desc}</p>
      </div>
    </div>
  );
};
