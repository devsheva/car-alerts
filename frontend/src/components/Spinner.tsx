import { Box, CircularProgress } from "@mui/material";

export default function Spinner() {
  return (
    <Box display={"flex"} justifyContent={"center"} alignItems={"center"}>
      <CircularProgress />
    </Box>
  );
}
