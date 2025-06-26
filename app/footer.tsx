import React from 'react';

const VortexAIChatFooter: React.FC = () => {
    return (
        <footer
            className="
        fixed bottom-0 left-0 w-full z-50      /* Ensures footer is fixed at viewport bottom, full width, on top */
        p-0                                     /* Removes all internal padding for minimal height */
        m-0                                     /* Removes all external margins */
        min-h-0                                 /* Ensures no minimum height forces extra space */
        bg-black/0                              /* Sets background to fully transparent (0% black opacity) */
        /* backdrop-blur-sm class is REMOVED to eliminate blur */
        text-gray-400                           /* Light gray text */
        text-[0.6rem]                           /* Very small font size for compactness */
        border-t border-gray-700/0              /* Top border is also set to fully transparent */
        flex flex-col items-center justify-center /* Centers content horizontally and stacks lines vertically */
        leading-none                            /* Removes extra line height within paragraphs for ultimate compactness */
      "
        >
            <p className="text-center leading-none m-0">
                Vortex AI Chat sometimes makes mistakes. Please check important information before acting on our app.
            </p>

            <p className="
        text-[0.5rem]                           /* Even tinier font size for the second line */
        text-gray-500                           /* Slightly darker gray for subtlety */
        text-center leading-none m-0            /* Center align, no line height, no margin */
      ">
                Tip: Always verify critical information. &copy; {new Date().getFullYear()} Vortex AI.
            </p>
        </footer>
    );
};

export default VortexAIChatFooter;