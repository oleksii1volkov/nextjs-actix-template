import { Box, Grid, Typography } from "@mui/material";
import { DateTimePicker } from "@mui/x-date-pickers";

const DateTimeRangePicker: React.FC = () => {
    return (
        // <Box>
        //     <Grid container spacing={2} alignItems="center">
        //         <Grid item xs={1}>
        //             <Typography variant="subtitle1" align="center">
        //                 from
        //             </Typography>
        //         </Grid>
        //         <Grid item xs={5}>
        //             <DateTimePicker />
        //         </Grid>
        //         <Grid item xs={1}>
        //             <Typography variant="subtitle1" align="center">
        //                 to
        //             </Typography>
        //         </Grid>
        //         <Grid item xs={5}>
        //             <DateTimePicker />
        //         </Grid>
        //     </Grid>
        // </Box>

        <Box display="flex" alignItems="center" gap={2}>
            <Typography variant="subtitle1" align="center">
                from
            </Typography>
            <DateTimePicker />
            <Typography variant="subtitle1" align="center">
                to
            </Typography>
            <DateTimePicker />
        </Box>
    );
};

export default DateTimeRangePicker;
