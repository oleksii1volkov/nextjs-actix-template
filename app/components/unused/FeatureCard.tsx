import React from "react";
import PropTypes from "prop-types";
import {
    Card,
    CardBody,
    Typography,
    IconButton,
} from "@material-tailwind/react";

export type FeatureCardColor =
    | "blue-gray"
    | "gray"
    | "brown"
    | "deep-orange"
    | "orange"
    | "amber"
    | "yellow"
    | "lime"
    | "light-green"
    | "green"
    | "teal"
    | "cyan"
    | "light-blue"
    | "blue"
    | "indigo"
    | "deep-purple"
    | "purple"
    | "pink"
    | "red";

type FeatureCardProps = {
    color?: FeatureCardColor;
    icon: React.ReactNode;
    title: string;
    description: React.ReactNode;
};

export function FeatureCard({
    color = "blue",
    icon,
    title,
    description,
}: FeatureCardProps) {
    return (
        <Card className="rounded-lg shadow-lg shadow-gray-500/10">
            <CardBody className="px-8 text-center">
                <IconButton
                    variant="gradient"
                    size="lg"
                    color={color}
                    className="pointer-events-none mb-6 rounded-full"
                >
                    {icon}
                </IconButton>
                <Typography variant="h5" className="mb-2" color="blue-gray">
                    {title}
                </Typography>
                <Typography className="font-normal text-blue-gray-600">
                    {description}
                </Typography>
            </CardBody>
        </Card>
    );
}

FeatureCard.propTypes = {
    color: PropTypes.oneOf([
        "blue-gray",
        "gray",
        "brown",
        "deep-orange",
        "orange",
        "amber",
        "yellow",
        "lime",
        "light-green",
        "green",
        "teal",
        "cyan",
        "light-blue",
        "blue",
        "indigo",
        "deep-purple",
        "purple",
        "pink",
        "red",
    ]),
    icon: PropTypes.node.isRequired,
    title: PropTypes.string.isRequired,
    description: PropTypes.node.isRequired,
};

FeatureCard.displayName = "/app/components/FeatureCard.jsx";

export default FeatureCard;
