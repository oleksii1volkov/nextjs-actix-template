import React from "react";
import PropTypes from "prop-types";
import { Card, Avatar, Typography } from "@material-tailwind/react";

type TeamCardProps = {
    img: string,
    name: string,
    position?: string,
    socials?: React.ReactElement | null,
};

export function TeamCard({
    img,
    name,
    position = "",
    socials = null,
}: TeamCardProps) {
    return (
        <Card color="transparent" shadow={false} className="text-center">
            <Avatar
                src={img}
                alt={name}
                size="xxl"
                variant="rounded"
                className="h-full w-full shadow-lg shadow-gray-500/25"
            />
            <Typography variant="h5" color="blue-gray" className="mt-6 mb-1">
                {name}
            </Typography>
            {position && (
                <Typography className="font-bold text-blue-gray-500">
                    {position}
                </Typography>
            )}
            {socials && <div className="mx-auto mt-5">{socials}</div>}
        </Card>
    );
}

TeamCard.propTypes = {
    img: PropTypes.string.isRequired,
    name: PropTypes.string.isRequired,
    position: PropTypes.string,
    socials: PropTypes.node,
};

TeamCard.displayName = "/app/components/TeamCard.jsx";

export default TeamCard;
