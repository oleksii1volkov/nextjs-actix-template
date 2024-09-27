import PropTypes from "prop-types";
import { Typography } from "@material-tailwind/react";

type PageTitleProps = {
    section: string;
    heading: string;
    children: React.ReactNode;
};

export function PageTitle({ section, heading, children }: PageTitleProps) {
    return (
        <div className="mx-auto w-full px-4 text-center lg:w-6/12">
            <Typography variant="lead" className="font-semibold">
                {section}
            </Typography>
            <Typography variant="h2" color="blue-gray" className="my-3">
                {heading}
            </Typography>
            <Typography variant="lead" className="text-blue-gray-500">
                {children}
            </Typography>
        </div>
    );
}

PageTitle.propTypes = {
    section: PropTypes.string.isRequired,
    heading: PropTypes.string.isRequired,
    children: PropTypes.node.isRequired,
};

PageTitle.displayName = "/app/components/PageTitle.jsx";

export default PageTitle;
