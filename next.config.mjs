import withBundleAnalyzer from "@next/bundle-analyzer";

const nextConfig = {
    reactStrictMode: true,
    trailingSlash: true,

    // Export settings
    //output: "export",

    // Optimization settings
    images: {
        unoptimized: true,
    },
};

const bundleAnalyzerConfig = withBundleAnalyzer({
    enabled: process.env.ANALYZE === "true",
});

export default { ...nextConfig, ...bundleAnalyzerConfig };
