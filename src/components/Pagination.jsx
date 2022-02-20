import React from "react";

export const Pagination = ({ total, limit, onPageChanged, selectedPage }) => {
  const [selected, setSelected] = React.useState(selectedPage);
  const rest = parseInt(total % limit) === 0 ? 0 : 1;
  const pagesCount = parseInt(total / limit) + rest;
  const pages = Array.from(Array(pagesCount).keys());

  return (
    <div className="flex mt-8 mb-2 justify-center">
      {pages.length > 1 &&
        pages.map((page, index) => (
          <div
            key={index}
            className={`${
              selected === page + 1 ? "bg-orange-600" : "bg-transparent"
            } flex mx-2 rounded-full border-2 border-orange-600 font-semibold justify-center items-center w-12 h-12 text-white cursor-pointer`}
            onClick={() => {
              setSelected(page + 1);
              onPageChanged(page + 1);
            }}
          >
            {page + 1}
          </div>
        ))}
    </div>
  );
};
