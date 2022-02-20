import { Menu } from "@headlessui/react";
import { ChevronDownIcon } from "@heroicons/react/solid";

export default function Dropdown({ title, options, selected }) {
  return (
    <Menu as="div" className="relative inline-block text-left">
      <Menu.Button
        className="inline-flex border-4 px-4 py-2 rounded-lg font-semibold uppercase text-white group transition ease-in-out duration-200 undefined border-orange-500">
        {selected ? (
          <>{selected}</>
        ) : (
          <>{title}</>
        )}
        <ChevronDownIcon className="-mr-1 ml-2 h-5 w-5" aria-hidden="true" />
      </Menu.Button>
      <Menu.Items
        className="origin-top-right absolute right-0 w-40 rounded-md bg-orange-600">
        <div className="py-1">
          {options?.map((option, index) => (
            <Menu.Item key={index}>
              <div className={`hover:bg-purple-800 transition ${selected === option.title ? "bg-purple-700" : ""}`}>
                <button
                  className="font-semibold text-left px-4 py-2 block w-full"
                  onClick={option.onClick}>{option.title}</button>
              </div>
            </Menu.Item>
          ))}
        </div>
      </Menu.Items>
    </Menu>
  );
}
