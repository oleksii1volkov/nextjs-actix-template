import Box from "@mui/material/Box";
import Button from "@mui/material/Button";
import Card from "@mui/material/Card";
import Typography from "@mui/material/Typography";

import DateTimeRangePicker from "./mui/DateTimeRangePicker";
import PlaceSelect from "./PlaceSelect";

const TourSearch: React.FC = () => {
    return (
        <Card className="w-full shadow-lg">
            <Box
                display="flex"
                justifyContent="center"
                alignItems="center"
                gap={2}
                p={6}
            >
                <Typography variant="subtitle1">Where</Typography>
                <PlaceSelect />
                <DateTimeRangePicker />
                <Button variant="contained">Search</Button>
            </Box>
        </Card>
    );
};

export default TourSearch;
