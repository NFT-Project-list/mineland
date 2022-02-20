export const formatId = (token_id) => {
  let token = token_id.split("-");
  return token[0].toUpperCase() + token[1];
};

export const statusColorBorderMap = (status) => {
  let result = "border-gray-500";
  if (status === "Medium" || status === "UnCommon") {
    result = "border-blue-500";
  } else if (status === "Large" || status === "Legendary") {
    result = "border-violet-500";
  } else if (status === "Rare") {
    result = "border-rose-500"
  }
  return result;
}
