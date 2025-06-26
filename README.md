Deploy your private Gemini application for free with one click, supporting Gemini 1.5 Pro, Gemini 1.5 Flash, Gemini Pro, and Gemini Pro Vision models.

English · 简体中文



<h3 class="h3">you can download this in the web app if you run it in the localhost:3000 If you use chrome </h3>

 


This section demonstrates the clean and intuitive user interface of Vorttertex AI Chat. The design prioritizes ease of use, ensuring that both new and experienced users can quickly navigate and interact with the application. The layout is responsive, adapting seamlessly to various screen sizes, from mobile devices to large desktop monitors, ensuring a consistent and pleasant user experience across all platforms. The visual aesthetics are modern and minimalist, reducing cognitive load and focusing the user's attention on the core interaction with the AI.

Vortertex AI Chat offers a simple interface that supports comprehensive image recognition and natural voice conversation capabilities. This fusion of visual and auditory input allows for more dynamic and intuitive interactions, making the AI feel more like a true assistant. Users can effortlessly upload images and ask contextual questions, or engage in spoken dialogues for hands-free operation.

This screenshot highlights the powerful multi-modal capabilities of Vorttertex AI Chat. It clearly shows the direct integration with Gemini 1.5 and Gemini 2.0 models, which are at the forefront of AI innovation. These models enable the application to process and understand various data types beyond just conventional text. This includes complex image inputs, allowing the AI to perform advanced visual recognition tasks such as object identification, scene analysis, and even text extraction from images. The application's responsive design ensures that this rich functionality is accessible and well-presented across all devices, from powerful desktop environments to portable mobile platforms, providing a consistent and high-quality user experience.

Vorttertex AI Chat fully supports Gemini 1.5 and Gemini 2.0 multimodal models. This means the AI can interpret and generate responses based on a combination of different input formats simultaneously. For example, you can seamlessly combine text with an image, or provide a voice input alongside a video clip, and the AI will process these diverse inputs holistically. This leads to more contextually rich, accurate, and dynamic interactions, transforming the way users engage with artificial intelligence.

The application supports a robust plugin architecture, featuring built-in Web search, Web reader, Arxiv search, Weather, and other practical plugins. This modular design significantly extends the AI's core capabilities, allowing it to perform a wide array of specialized tasks. The Web search plugin enables the AI to retrieve real-time information from the internet, ensuring its knowledge base is always current. The Web reader can summarize long articles or extract key information from web pages, saving users valuable time. The Arxiv search plugin is invaluable for researchers, providing direct access to a vast repository of scientific papers. The Weather plugin offers instant and accurate local or global weather forecasts. The extensibility of this plugin system means that new functionalities can be integrated effortlessly, adapting Vorttertex AI Chat to an ever-evolving range of user needs and use cases. Each plugin is designed for seamless integration and ease of use, enhancing the overall utility and versatility of the AI assistant.

The Multimodal Live API feature is a cornerstone of Vorttertex AI Chat, providing an unparalleled interactive experience that blurs the lines between human and AI communication. This visual representation showcases the fluid integration of real-time voice and video capabilities, allowing users to engage in natural, dynamic conversations with the AI. The system is engineered for low-latency performance, which means interactions feel incredibly fluid and responsive, closely mimicking the spontaneity and natural rhythm of human conversation. This level of responsiveness is critical for applications demanding immediate feedback and complex contextual understanding.

Vorttertex AI Chat supports the Multimodal Live API, enabling a remarkably smooth voice and video experience. This advanced integration allows the AI to not only understand the explicit content of spoken language but also interpret subtle nuances in tone, intonation, and even visual cues from video streams. This comprehensive understanding leads to more contextually aware, accurate, and empathetic responses from the AI. The system's optimization for real-time processing ensures that there are no noticeable delays, which is paramount for immersive interactive experiences. This cutting-edge capability firmly positions Vorttertex AI Chat at the forefront of interactive AI applications, bridging the gap between traditional human communication and advanced artificial intelligence, fostering a more natural and engaging user experience.

The cross-platform application client, as depicted in this screenshot, is a powerful and indispensable addition for desktop users who prioritize efficiency and accessibility. This client is engineered to integrate seamlessly into the operating system's menu bar (on macOS and Linux) or system tray (on Windows). This strategic placement ensures that Vorttertex AI Chat's powerful functionalities are always just a click away, eliminating the need to open a full browser window or navigate through multiple tabs. This "always-on" availability significantly enhances productivity, allowing users to leverage AI assistance instantly, whether they are deeply immersed in coding, meticulously crafting documents, managing complex projects, or simply browsing the web.

Vorttertex AI Chat offers a lightweight, cross-platform application client that supports a permanent menu bar presence, effectively doubling your work efficiency. The client is designed for minimal resource consumption, typically occupying only about ~4MB of disk space, ensuring it runs efficiently in the background without noticeably impacting system performance. This native application provides a highly responsive and fluid user experience, mirroring the look and feel of other desktop applications. Its persistent presence in the menu bar allows for rapid AI queries, quick access to conversation history, and immediate activation of features, making it an indispensable tool for daily workflows and significantly enhancing the overall user experience by reducing friction in accessing AI capabilities.

Note: If you encounter problems during the use of the project, you can check the known problems and solutions in the FAQ section. This section serves as a comprehensive knowledge base for troubleshooting common issues and provides step-by-step solutions. We highly recommend consulting it as your first resource if you encounter any unexpected behavior, errors, or require clarification on specific functionalities. The FAQ is continuously updated based on user feedback and emerging trends in AI deployment.

Features
Vorttertex AI Chat is equipped with a comprehensive suite of features engineered to provide a seamless, efficient, and powerful AI interaction experience. Each feature is meticulously crafted with user convenience and performance in mind, ensuring a robust and intuitive conversational framework.

Deploy for free with one-click on Vercel in under 1 minute: This is arguably one of the most compelling features. Vercel offers an incredibly streamlined deployment process, enabling users to launch their private Vorttertex AI Chat instance with minimal effort. This involves clicking a single button, which triggers an automated process of cloning the repository, installing all necessary dependencies, building the application, and deploying it globally onto Vercel's Edge Network. This entire sequence typically completes within a minute, providing immediate access to your personalized AI chat environment without the complexities of manual server setup or configuration. The free tier of Vercel makes this accessible to everyone, fostering widespread adoption and experimentation.

<!--
Detailed breakdown of Vercel one-click deployment:
1. User clicks the "Deploy with Vercel" button.
2. Vercel initiates a new project creation process.
3. The specified GitHub repository (Adevloper152/Vorttertex-ai-chat) is cloned.
4. Required Node.js and pnpm dependencies are automatically installed.
   Example: pnpm install --frozen-lockfile
5. The Next.js application is built for production.
   Example: pnpm build
6. Environment variables (like GEMINI_API_KEY, ACCESS_PASSWORD) are securely configured based on user input during the Vercel setup wizard.
   These are crucial for the application's functionality.
7. The built application is deployed to Vercel's global CDN (Content Delivery Network).
8. A unique URL is assigned to the deployed application, accessible worldwide.
9. The user is redirected to their newly deployed Vorttertex AI Chat instance.

This entire flow is designed for speed and simplicity, abstracts away infrastructure concerns,
and leverages Vercel's robust platform for scalability and reliability.
-->

Lightweight Cross-Platform Desktop Client (~4MB): The desktop client is a testament to efficient software engineering. Its remarkably small footprint ensures that it is lightweight and consumes minimal system resources, making it suitable for a wide range of hardware configurations without impacting performance. This client is designed to enhance workflow efficiency by residing conveniently in the operating system's menu bar (macOS, Linux) or system tray (Windows). Its permanent presence allows users to quickly invoke the AI chat interface, initiate new conversations, or access settings without disrupting their current tasks. This direct accessibility significantly improves daily productivity by making AI assistance an integrated part of the desktop experience, rather than a separate application that needs to be manually launched.

<!--
The small size of the client is achieved through:
- Efficient bundling with Tauri, which is a Rust-based framework for building cross-platform binaries.
- Minimal dependencies in the client application.
- Optimized asset loading and resource management.

Benefits of a small client:
- Faster download and installation.
- Lower memory footprint.
- Quicker launch times.
- Less impact on overall system performance.
- Ideal for users with limited storage or older hardware.

The client's persistent menu bar/tray icon ensures "always-on" access, making it a true utility.
-->

Supports multi-modal models and can understand images, videos, audios and some text documents: Vorttertex AI Chat leverages the advanced capabilities of Gemini's multi-modal architecture. This means the AI isn't limited to processing text; it can simultaneously interpret and respond to information presented in various formats. Users can provide images (e.g., photos, diagrams), video clips, audio recordings (e.g., spoken questions, environmental sounds), and even complex text documents (e.g., PDFs, rich text formats, code snippets). The AI intelligently synthesizes information from these diverse inputs to provide comprehensive and contextually relevant responses, offering a more natural and sophisticated interaction compared to traditional text-only chatbots. This capability opens up a vast array of use cases, from analyzing visual data to transcribing and summarizing audio content.

<!--
Multimodal input examples:
- Image + Text: "What is this building in the picture, and what's its history?"
- Video + Text: "Summarize the key points discussed in this video clip."
- Audio + Text: "Transcribe this audio recording and then answer questions about its content."
- Document + Text: "Analyze this scientific paper (PDF) and explain the methodology used."

The underlying Gemini models handle the complex task of fusing these different data types.
This feature is essential for real-world applications where information often comes in mixed formats.
-->

Talk mode: Let you talk directly to Gemini, support Multimodal Live API: The "Talk mode" transforms Vorttertex AI Chat into a truly interactive conversational partner. This feature enables users to engage in direct, spoken dialogue with the Gemini AI, providing a hands-free and highly intuitive interaction method. The integration of the Multimodal Live API ensures that these voice conversations are incredibly smooth and responsive, minimizing latency and making the interaction feel remarkably natural. This allows for fluid back-and-forth exchanges, as if you were speaking to another person. The system intelligently processes spoken queries, generates spoken responses, and maintains conversational context, providing an immersive and efficient communication channel.

<!--
Key aspects of Talk Mode and Multimodal Live API:
- Real-time Speech-to-Text (STT) for user input.
- Real-time Text-to-Speech (TTS) for AI responses.
- Low-latency audio streaming for a fluid conversation flow.
- Contextual understanding maintained across spoken turns.
- Ideal for brainstorming, dictation, quick queries, or accessibility needs.
- The Multimodal Live API handles the continuous streaming of audio and potentially video,
  allowing for ongoing, dynamic interpretation by the Gemini model.
-->

Visual recognition allows Gemini to understand the content of the picture: A dedicated and powerful feature, visual recognition in Vorttertex AI Chat enables the Gemini model to go beyond simply identifying objects in an image. It empowers the AI to deeply analyze and comprehend the nuanced content, context, and relationships within a picture. Users can upload various types of images—photographs, diagrams, charts, screenshots—and ask specific questions about them. The AI can then identify objects, scenes, text, emotions, and even infer activities or situations depicted in the visual data. This capability is invaluable for tasks requiring visual data interpretation, such as describing complex graphs, identifying landmarks, or providing detailed explanations of visual concepts, making the AI a truly "seeing" assistant.

<!--
Examples of Visual Recognition capabilities:
- Image of a dog: "What breed is this dog?" or "Is this dog happy?"
- Screenshot of code: "Explain what this JavaScript function does." or "Find the error in this Python snippet."
- Chart/Graph: "What trends are shown in this data?" or "What are the values for Q3 2024?"
- Medical image: "Describe what this X-ray shows." (Note: AI is not a substitute for medical professionals)
- Scene analysis: "Describe the atmosphere of this landscape photo."

This is powered by Gemini Pro Vision and Gemini 1.5 models, which are trained on vast datasets of images and their descriptions.
The integration ensures secure and efficient processing of visual data directly through the API.
-->

Assistant market with hundreds of selected system instructions: Vorttertex AI Chat features a curated "Assistant Market" that provides users with a vast library of pre-configured AI personalities and specialized functions. These "system instructions" or "prompts" are essentially pre-defined directives that guide the AI's behavior, tone, and knowledge focus. Instead of crafting complex prompts from scratch, users can select an assistant tailored for specific tasks—such as a "Creative Writer," "Technical Explainer," "Code Debugger," "Travel Planner," or "Language Tutor." This market empowers users to quickly switch contexts and optimize the AI's performance for diverse needs, significantly enhancing its versatility and ease of use. Each instruction is carefully vetted to ensure quality and effectiveness.

<!--
Benefits of the Assistant Market:
- **Time-saving**: No need to craft complex initial prompts.
- **Consistency**: Ensures the AI adheres to a specific role or persona.
- **Task-specific optimization**: AI's responses are more focused and relevant to the chosen role.
- **Discovery**: Helps users explore new ways to leverage the AI.
- **Accessibility**: Makes advanced prompting techniques accessible to all users.

The "system instruction" acts as a foundational context for the AI, influencing all subsequent turns in a conversation.
It's akin to giving the AI a role to play, ensuring its responses align with that role.
-->

Support plugins, with built-in Web search, Web reader, Arxiv search, Weather and other practical plugins: The plugin system is a core architectural strength of Vorttertex AI Chat, enabling seamless integration of external tools and data sources. Beyond the core AI capabilities, these plugins extend the AI's utility by providing access to real-world information and specialized functions. The Web search plugin allows the AI to perform live internet queries, ensuring it has access to the most current information. The Web reader can process and summarize content from URLs, making it easy to digest long articles or research papers. The Arxiv search plugin is particularly useful for academic and research-oriented users, providing direct access to the arXiv preprint repository. The Weather plugin fetches real-time weather forecasts based on specified locations. This modular design ensures that Vorttertex AI Chat remains adaptable and extensible, capable of integrating new tools as user needs evolve or as new APIs become available. This significantly enhances the AI's practical applicability in diverse real-world scenarios.

<!--
Technical detail on plugins (Function Calling concept):
- Plugins are implemented using Gemini's "Function Calling" capability.
- The AI model, when presented with a user query, can decide to call a predefined tool (plugin).
- Example flow:
    1. User asks: "What's the weather like in San Diego?"
    2. AI recognizes intent and determines it needs the 'weather' plugin.
    3. AI constructs a "tool call" (e.g., {'function': 'get_weather', 'parameters': {'location': 'San Diego'}}).
    4. The application executes this tool call on the backend, fetching data from an external weather API.
    5. The result (e.g., {'temperature': 25, 'conditions': 'Sunny'}) is sent back to the AI.
    6. AI synthesizes a natural language response based on the fetched data: "The weather in San Diego is sunny with a temperature of 25 degrees Celsius."
- This architecture makes the AI highly versatile and capable of interacting with the external world.
- New plugins can be added by simply defining their function signatures and providing their execution logic.
-->

Conversation list, so you can keep track of important conversations or discuss different topics with Gemini: The conversation list feature provides an intuitive and persistent way to manage your interactions with Vorttertex AI Chat. It allows users to create, store, and easily switch between multiple distinct conversation threads. This is crucial for maintaining context when discussing different topics, preventing overlap, and ensuring that important information is not lost. Users can create new conversations for separate projects, research topics, or personal inquiries, keeping their AI interactions organized. This feature acts as a digital memory for your AI dialogues, allowing you to pick up exactly where you left off, review past advice, or revisit creative outputs.

<!--
Conversation list management details:
- Each conversation is typically stored locally in the user's browser (e.g., localStorage or IndexedDB).
- Provides options to:
    - Create new conversations.
    - Rename existing conversations.
    - Delete conversations.
    - Pin important conversations for quick access.
    - Search through conversation titles or content.
- Contextual memory is maintained within each specific conversation thread, optimizing AI responses for that topic.
- This feature is fundamental for practical, long-term use of an AI assistant.
-->

Artifact support, allowing you to modify the conversation content more elegantly: Artifact support is an innovative feature that provides users with fine-grained control over the conversation flow and content. Unlike traditional chat interfaces where past messages are immutable, artifacts allow users to interact with and modify specific parts of the conversation dynamically. This might include editing previous AI responses for clarity, refining user prompts to steer the AI in a new direction, or even injecting new information into the historical context. This "editable memory" enhances the iterative nature of AI interaction, making it easier to correct misunderstandings, iterate on creative outputs, or adjust the AI's focus without starting a new conversation from scratch. It promotes a more collaborative and refined dialogue experience.

<!--
Examples of Artifact support:
- **Editing AI responses**: If the AI provides a good but slightly off response, the user can edit it directly to refine the "truth" for future turns.
- **Rewriting user prompts**: If a prompt was unclear, it can be rephrased to improve AI understanding for subsequent interactions.
- **Injecting information**: A user might add a piece of data to an earlier part of the conversation that was missing, providing more context to the AI.
- **"Forking" a conversation point**: Create a new branch of conversation from a specific artifact, exploring an alternative path.

This feature fundamentally improves the ability to "steer" the AI and makes the conversation a living document rather than a fixed transcript.
It leverages the concept of mutable chat history, which is critical for complex tasks.
-->

Full Markdown support: KaTex formulas, code highlighting, Mermaid charts, etc.: Vorttertex AI Chat provides comprehensive support for Markdown syntax, significantly enhancing the readability and versatility of both user input and AI responses. This includes advanced rendering capabilities for:

KaTeX formulas: Mathematical equations and scientific notations can be rendered beautifully and accurately, making the AI ideal for academic, engineering, and scientific discussions.

<!-- Example KaTeX usage: -->
$E=mc^2$
$$ \int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi} $$

Code highlighting: Code snippets shared by users or generated by the AI are automatically highlighted with syntax coloring for various programming languages, improving readability and making it easier to analyze and debug code.

# Example Python code highlighting:
def factorial(n):
    if n == 0:
        return 1
    else:
        return n * factorial(n-1)

Mermaid charts: Complex diagrams, flowcharts, sequence diagrams, and gantt charts can be directly rendered from simple Markdown-like text, providing a powerful tool for visual communication and planning within the chat interface.

graph TD
    A[Start] --> B{Is it true?};
    B -- Yes --> C[Do something];
    B -- No --> D[Do nothing];
    C --> E[End];
    D --> E;

This rich Markdown support ensures that complex information, whether it's code, data, or technical diagrams, is presented clearly and professionally, facilitating more effective communication and understanding.

<!--
Benefits of extensive Markdown support:
- **Enhanced readability**: Structured text is easier to parse.
- **Professional presentation**: Makes AI responses look polished.
- **Versatile communication**: Supports various forms of content (code, math, diagrams).
- **Interoperability**: Markdown is a widely used format, making content transferable.
- **User and AI consistency**: Both sides can leverage the same rich formatting.
-->

Automatically compress contextual chat records to save Tokens while supporting very long conversations: This is a crucial optimization for managing API costs and maintaining long-term conversational memory. As conversations grow longer, the amount of "context" sent to the AI model increases, consuming more tokens and incurring higher costs. Vorttertex AI Chat intelligently compresses older parts of the conversation, summarizing or truncating less critical information while retaining the essential context. This ensures that the AI maintains coherence and memory over extended dialogues without unnecessarily inflating token usage. This intelligent compression mechanism allows for truly very long conversations, making the AI a reliable partner for ongoing projects or deep, multi-turn discussions.

<!--
How contextual compression typically works:
- **Sliding Window**: Only the most recent 'N' turns are sent fully.
- **Summarization**: Older turns or groups of turns are summarized into a concise block of text.
- **Retrieval Augmented Generation (RAG) principles (future potential)**: Relevant past conversations might be retrieved and injected as context based on current query.
- **Token Limits**: AI models have maximum token limits for context (e.g., Gemini 1.5 Flash has a very large context window, but optimizing it is still good practice).
- **Cost Efficiency**: Directly reduces the number of input tokens sent to the API, thus reducing operational costs.
- **Coherence Preservation**: Ensures the AI "remembers" the core elements of the conversation even when it's very long, leading to a more consistent and satisfying interaction.
-->

Privacy and security, all data is saved locally in the user's browser: Vorttertex AI Chat prioritizes user privacy and data security. A fundamental design principle is that all conversation data and user settings are stored exclusively on the client-side, within the user's local browser storage (e.g., IndexedDB or localStorage). This means no sensitive conversational data ever leaves your device or is stored on external servers, providing an exceptional level of privacy. Users have complete control over their data, and it is not shared with or accessible by the application's developers or any third parties. This local storage approach ensures that your private conversations truly remain private, fostering trust and security.

<!--
Implications of local data storage:
- **No server-side database**: Simplifies infrastructure and eliminates a common point of data breaches.
- **Enhanced privacy**: User data remains entirely on their device.
- **GDPR/CCPA compliance by design**: Reduces complex data handling compliance requirements for users.
- **Offline access potential**: While the AI requires internet, local data can be accessed offline.
- **Backup responsibility**: Users are responsible for backing up their browser profile/data if they wish to migrate conversations.
- **Cross-device synchronization (if desired, future feature)**: Would require explicit user opt-in and encryption for cloud sync.
-->

Support PWA, can run as an application: Vorttertex AI Chat is designed as a Progressive Web Application (PWA). This means it offers a web experience that has the look and feel of a native application. Users can "install" the PWA directly from their browser to their device's home screen (on mobile) or desktop (on desktop operating systems), bypassing traditional app store installations. Once installed, it can run in a standalone window, often with offline capabilities for the UI, and offers features like push notifications (if implemented). This provides a seamless, fast, and reliable user experience, combining the best aspects of web and native applications. It's always up-to-date since it's a web application, yet it provides an app-like convenience.

<!--
Benefits of PWA support:
- **Installability**: Can be added to home screen/desktop.
- **Offline capabilities**: Core UI can load even without internet.
- **App-like experience**: Runs in its own window, no browser chrome.
- **Fast loading**: Leverages service workers for caching.
- **Discoverable**: Accessible via URLs.
- **Low friction**: No app store approval processes.
- **Automatic updates**: Users always have the latest version.

This makes Vorttertex AI Chat highly accessible and convenient across a variety of devices.
-->

Well-designed UI, responsive design, supports dark mode: The user interface of Vorttertex AI Chat is crafted with a strong emphasis on modern aesthetics and user experience principles. The well-designed UI features clean lines, intuitive navigation, and thoughtful visual hierarchy, ensuring that information is presented clearly and interactions are straightforward. The responsive design ensures that the application fluidly adapts to any screen size, from the smallest smartphone to the largest desktop monitor, maintaining optimal usability and visual appeal across all devices. Furthermore, full dark mode support is integrated, allowing users to switch to a darker color scheme. This not only reduces eye strain in low-light environments but also helps save battery life on devices with OLED screens. The attention to detail in the UI/UX ensures a pleasant, efficient, and personalized user experience.

<!--
UI/UX design principles applied:
- **Minimalism**: Clean layout, reduced clutter.
- **Accessibility**: High contrast for readability, keyboard navigation support.
- **Consistency**: Uniform design elements across the application.
- **Intuitiveness**: Features are easy to find and understand.
- **Aesthetics**: Modern color palette and typography.
- **Responsiveness details**:
    - Flexbox and CSS Grid for adaptable layouts.
    - Media queries for breakpoint-specific styling.
    - Scalable vector graphics (SVGs) for crisp icons.
- **Dark mode implementation**: CSS variables or Tailwind CSS variants for easy theming.
-->

Extremely fast first screen loading speed, supporting streaming response: Performance is a critical aspect of Vorttertex AI Chat. The application is highly optimized to achieve an extremely fast first screen loading speed, meaning users can start interacting with the AI almost instantaneously after navigating to the page. This is achieved through various web performance best practices, including code splitting, lazy loading, and efficient asset management. Complementing this, the application supports streaming responses from the Gemini API. Instead of waiting for the AI to generate a complete answer before displaying it, responses are streamed word-by-word or token-by-token. This creates a highly engaging and immediate user experience, as users can read the AI's answer as it's being generated, enhancing perceived responsiveness and making interactions feel more dynamic and conversational.

<!--
Technical details on performance optimization:
- **Next.js Static Site Generation (SSG) / Server-Side Rendering (SSR)**: Provides pre-rendered HTML for faster initial load.
- **Code Splitting**: Breaks down JavaScript bundles into smaller chunks loaded only when needed.
- **Image Optimization**: Using Next.js Image component for efficient image loading.
- **Critical CSS**: Inlining essential CSS for immediate styling.
- **Streaming Responses**:
    - Leveraging Server-Sent Events (SSE) or WebSockets for continuous data flow from API.
    - Front-end efficiently renders incoming tokens in real-time.
    - Provides a more dynamic and engaging user experience compared to waiting for a full response.
    - Reduces perceived latency for the user.
    - This is crucial for large language models to provide interactive feedback.
- **CDN deployment (Vercel/Cloudflare)**: Distributes assets globally for faster access.
-->

Static deployment supports deployment on any website service that supports static pages, such as GitHub Pages, Cloudflare, Vercel, etc.: Vorttertex AI Chat offers the flexibility of static deployment, which is a highly efficient and cost-effective method for hosting web applications. By running pnpm build:export, the project generates a set of static HTML, CSS, and JavaScript files that can be served directly by any web server or static hosting service. This includes popular platforms like GitHub Pages, Cloudflare Pages, and Vercel (which also supports hybrid deployments but excels at static sites). Static deployment offers numerous advantages, including enhanced security (no server-side runtime vulnerabilities), exceptional speed (content served directly from a CDN), and simplified maintenance (no backend server to manage). This makes Vorttertex AI Chat incredibly versatile and accessible for deployment in various environments, catering to diverse user needs and infrastructure preferences.

<!--
Advantages of Static Deployment:
- **Simplicity**: No complex server-side setup or database management.
- **Speed**: Files are served directly from a Content Delivery Network (CDN), minimizing latency.
- **Scalability**: Can handle sudden traffic spikes without performance degradation.
- **Security**: Reduced attack surface compared to dynamic applications.
- **Cost-effectiveness**: Often free or very low cost on many hosting platforms.
- **SEO Friendly**: Pre-rendered HTML is easily crawlable by search engines.

The `out` directory, generated by `pnpm build:export`, contains all necessary static assets.
Users simply need to upload the contents of this directory to their chosen static hosting provider.
-->

Multi-language support: English, 简体中文, 繁體中文, 日本語, 한국어, Español, Deutsch, Français, Português, Русский and العربية: Vorttertex AI Chat is designed for a global audience, offering extensive multi-language support (internationalization, or i18n). The application provides a user interface available in numerous languages, including English, Simplified Chinese, Traditional Chinese, Japanese, Korean, Spanish, German, French, Portuguese, Russian, and Arabic. This commitment to linguistic diversity ensures that users from various linguistic backgrounds can interact with the application comfortably and efficiently in their native language. The localization efforts cover all UI elements, messages, and settings, providing a truly tailored experience for a wide range of users worldwide. This broad language support enhances accessibility and promotes a more inclusive user community.

<!--
Technical implementation of Multi-language support:
- **i18next** or **react-i18next**: Common libraries for handling internationalization in React/Next.js applications.
- **JSON language files**: UI strings are stored in structured JSON files for each language (e.g., `en.json`, `zh-CN.json`).
- **Dynamic language switching**: Users can typically select their preferred language from settings.
- **Browser language detection**: The application might default to the user's browser language if supported.
- **Locale routing (Next.js)**: Potentially allows for language-specific URLs (e.g., `/en/chat`, `/zh-CN/chat`).

This feature is crucial for breaking down language barriers and making advanced AI technology accessible to a wider demographic.
-->

Roadmap
The development of Vorttertex AI Chat is guided by a clear and ambitious roadmap, focusing on continuous improvement and the introduction of groundbreaking features to enhance the AI chat experience. Each item on this roadmap represents a commitment to innovation and user-centric development.

[x] Reconstruct the topic square and introduce Prompt list: This milestone involves a complete overhaul of the AI's "topic square" (or prompt library) feature. The aim is to create a more intuitive and discoverable interface for managing and selecting system instructions or "prompts." The "Prompt list" will offer a more organized and searchable collection of pre-defined AI behaviors, making it easier for users to find and apply the perfect AI persona or function for their specific needs, thereby streamlining interaction initiation.

[x] Use Tauri to package desktop applications: This significant architectural decision involves transitioning the desktop client packaging from potentially Electron (or other methods) to Tauri. Tauri is a framework for building cross-platform binaries using web technologies (like Next.js) for the UI, but with a Rust-based backend for native system integration. This move aims to drastically reduce the size of the desktop application (as seen in the ~4MB footprint), improve performance, enhance security, and provide a more native feel compared to larger, less efficient alternatives.

[x] Implementation based on functionCall plugin: This core development integrates Gemini's Function Calling capabilities directly into the application's plugin system. This means the AI model itself can intelligently decide when to use an external tool (like a Web search, weather API, or Arxiv search) based on the user's query. This capability makes the AI significantly more powerful and versatile, allowing it to interact with the external world, retrieve real-time data, and perform actions beyond its core knowledge base, transforming it into a true "agent."

[x] Support conversation list: This crucial feature enables users to effectively manage multiple, distinct conversation threads. The "conversation list" provides a structured way to create, organize, rename, and switch between different chat sessions. This is vital for maintaining context across various topics or projects, preventing confusion, and allowing users to revisit specific discussions exactly where they left off. It turns the ephemeral nature of chat into a persistent, organized knowledge base.

[x] Support conversation export features: To ensure data portability and user control, this feature allows users to export their conversation histories. This typically includes options to export conversations in various formats (e.g., Markdown, plain text, JSON), making it easy for users to backup their dialogues, share them, or import them into other applications. This enhances data ownership and flexibility, providing users with complete control over their interaction history.

[x] Enable Multimodal Live API: This cutting-edge implementation integrates Google Gemini's Multimodal Live API. This API facilitates real-time, fluid voice and potentially video interactions with the AI. It minimizes latency in speech-to-text and text-to-speech conversions, creating a highly natural and responsive conversational experience. This enables true interactive dialogue, where the AI can understand subtle vocal cues and visual context in real-time, making interactions feel more intuitive and lifelike.

[ ] Support networked Deep Research mode: This upcoming feature aims to significantly enhance the AI's research capabilities. The "Deep Research mode" will allow the AI to perform more extensive and autonomous information gathering from the internet and other networked sources. This might involve multi-step search queries, cross-referencing information from multiple sources, summarizing findings, and presenting a synthesized report. This mode would transform the AI into a powerful research assistant, capable of generating comprehensive insights from vast amounts of online data.

[ ] Support local knowledge base: This future enhancement will enable users to provide the AI with their own local datasets or documents to use as a knowledge base. Instead of relying solely on the AI's pre-trained knowledge or real-time web searches, users could upload personal notes, company documents, specific research papers, or private data. The AI would then be able to answer questions and generate responses specifically based on this local, private information, making it an invaluable tool for personalized learning, business intelligence, or sensitive data analysis without needing to send that data to an external API.

Get Started
Getting your own Vorttertex AI Chat instance up and running is designed to be a straightforward and quick process. Follow these detailed steps to begin harnessing the power of Gemini AI.

Get your Gemini API Key: The very first step is to obtain your unique API key from Google AI Studio. This key is essential for your Vorttertex AI Chat application to communicate with Google's Gemini models.

Navigate to the Google AI Studio API Key page.

Sign in with your Google account.

If you don't have a key, click "Create API key in new project" or "Create API key in existing project."

Copy the generated API key. Keep this key secure and do not share it publicly. It's your personal access credential to the Gemini API.

<!--
Detailed steps for obtaining the API Key:
1. Open your web browser.
2. Go to the official Google AI Studio website for API keys.
   URL: https://aistudio.google.com/app/apikey
3. You will be prompted to log in using your Google account credentials. If you are already logged in, you will be directed to your API key dashboard.
4. On the API key dashboard, look for a button that says "Create API key" or similar.
   - If this is your first time, it might suggest creating a new project for the key.
   - If you have existing projects, you might have the option to associate it with one of them.
5. Click the button to generate the new API key.
6. Once generated, a string of characters (your API key) will be displayed. This is a crucial step.
7. Immediately copy this key to your clipboard.
8. It is highly recommended to store this key securely, perhaps in a password manager or a secure note.
   Never commit your API key directly into your code repository.
   The deployment steps will show you how to safely provide it as an environment variable.
-->

One-click deployment of the project to Vercel: Vercel provides the simplest and fastest way to deploy Vorttertex AI Chat. This method leverages Vercel's powerful platform to automate the entire deployment process from your GitHub repository.

Click the "Deploy with Vercel" button above. This will take you to the Vercel deployment page.

Vercel will prompt you to link your GitHub account (if you haven't already). This is necessary for Vercel to clone the repository.

You'll be asked to provide environment variables. Paste your copied GEMINI_API_KEY into the corresponding field.

Optionally, set an ACCESS_PASSWORD to protect your deployed application with a simple password.

Click "Deploy." Vercel will then automatically clone the repository, install dependencies, build the project, and deploy it globally.

Once the deployment is complete, you will be provided with a unique URL to access your live Vorttertex AI Chat application.

<!--
Detailed walkthrough for Vercel deployment:
1. **Click the Vercel Deploy Button**:
   This button contains a pre-configured URL that tells Vercel:
   - Which repository to clone: `https://github.com/Adevloper152/Vorttertex-ai-chat`
   - The desired project name on Vercel: `Vorttertex-ai-chat`
   - The environment variables to prompt for: `GEMINI_API_KEY` and `ACCESS_PASSWORD`
   - The repository name to create on Vercel: `Vorttertex-ai-chat`

2. **Connect GitHub (if not already connected)**:
   Vercel requires access to your GitHub account to fork or clone the repository.
   Follow the prompts to authorize Vercel with your GitHub account.
   You can choose to give access to all repositories or only specific ones.
   For this project, access to `Adevloper152/Vorttertex-ai-chat` or a fork of it is required.

3. **Configure Project and Environment Variables**:
   - **Project Name**: Vercel will suggest "Vorttertex-ai-chat". You can customize this if you wish.
   - **Root Directory**: Leave as default (`./`).
   - **Framework Preset**: Vercel will auto-detect Next.js.
   - **Environment Variables**:
     - **GEMINI_API_KEY**: Paste the API key you obtained in step 1.
       Example: `AIzaSyB-Your_Actual_Gemini_API_Key_Here_12345`
       This key is vital for the application to function. Without it, the AI will not be able to respond.
     - **ACCESS_PASSWORD (optional)**: Set a strong password here if you want to restrict access to your deployed application.
       If left empty, anyone with the URL can access it.
       Example: `mySecureP@ssw0rd!`
       Remember this password, as you'll need it to log in after deployment.

4. **Deploy**:
   Click the "Deploy" button. Vercel will now perform the following automated tasks:
   - Clone your fork of the repository.
   - Install project dependencies (`pnpm install`).
   - Build the Next.js application for production (`pnpm build`).
   - Deploy the built application to Vercel's global CDN.
   - Assign a unique `.vercel.app` domain to your project.

5. **Access Your Application**:
   Once the deployment process shows as "Complete," Vercel will provide you with the live URL.
   Click on this URL to open your very own private Vorttertex AI Chat application.
   If you set an `ACCESS_PASSWORD`, you'll be prompted to enter it before gaining access.

This entire process is designed to be highly automated, taking away the complexities of server management and allowing you to focus on using the AI.
-->

Start using: Once your application is deployed and accessible via its URL, you can begin interacting with your private Gemini AI. Explore the features, try different models, utilize the plugins, and enjoy a personalized AI chat experience.

Deploy to Cloudflare
Vorttertex AI Chat also supports deployment to Cloudflare Pages, a powerful platform for hosting static sites and front-end applications. Deploying to Cloudflare Pages offers benefits such as a robust global CDN, strong security, and custom domain support.

However, the deployment process for Cloudflare Pages requires a few manual steps beyond a simple one-click button due to specific build configurations. You will need to configure the build command and output directory manually in the Cloudflare Pages dashboard after connecting your GitHub repository.

Please follow the detailed instructions provided in the dedicated guide: How to deploy to Cloudflare Page. This document will walk you through connecting your GitHub repository, setting up the build configuration, and deploying your application successfully on Cloudflare Pages.

<!--
Cloudflare Pages deployment typically involves:
1. Connecting your GitHub account to Cloudflare Pages.
2. Selecting your `Adevloper152/Vorttertex-ai-chat` repository.
3. Configuring the build settings:
   - **Build command**: `pnpm build:export` (This command generates the static `out` directory).
   - **Build output directory**: `out` (This is where the static files are placed).
   - **Root directory**: `./`
4. Adding environment variables in the Cloudflare Pages settings, similar to Vercel (e.g., GEMINI_API_KEY, ACCESS_PASSWORD).
5. Initiating the build and deployment.

Cloudflare Pages is an excellent alternative for those who prefer Cloudflare's ecosystem or need specific features it provides.
It offers blazing-fast static asset delivery through its global network.
-->

Updating Code
To keep your deployed Vorttertex AI Chat instance up-to-date with the latest features, bug fixes, and performance improvements from the upstream repository, you'll need to periodically synchronize your forked repository.

If you want to update instantly, you can check out the GitHub documentation to learn how to synchronize a forked project with upstream code. This process typically involves:

Adding the upstream remote: Point your local repository to the original u14app/vortex-ai-chat repository.

git remote add upstream https://github.com/u14app/vortex-ai-chat.git

Fetching upstream changes: Download the latest changes from the original repository without merging them yet.

git fetch upstream

Merging changes: Switch to your main branch (e.g., main or master) and merge the upstream changes.

git checkout main
git merge upstream/main

Pushing changes to your fork: Upload the synchronized changes to your GitHub fork.

git push origin main

Once your GitHub fork is updated, your Vercel or Cloudflare Pages deployment (if configured for continuous deployment) will automatically detect the changes and trigger a new build and deployment, ensuring your application is always running the latest version.

You can star or watch this project on GitHub, or follow the author, to receive timely release notifications. This will ensure you're always aware when new updates are available, allowing you to synchronize your fork and benefit from the latest improvements to Vorttertex AI Chat.

Environment Variables
Environment variables are crucial for configuring your Vorttertex AI Chat instance. They allow you to securely provide sensitive information like API keys and customize application behavior without embedding these values directly into the codebase. This enhances both security and flexibility.

GEMINI_API_KEY (optional)
Purpose: This variable holds your Google Gemini API key. It's the primary credential that allows your Vorttertex AI Chat application to authenticate with and make requests to Google's Gemini models (Gemini 1.5 Pro, Gemini 1.5 Flash, Gemini Pro, Gemini Pro Vision).

Requirement: This variable is required if you intend to enable and use the server-side API proxy provided by the application. While the frontend can also directly use an API key (if configured), providing it on the server (e.g., through Vercel or Docker) is generally more secure as it prevents the key from being exposed client-side.

Frontend Impact: Important Note: Setting this environment variable on the server-side (e.g., in Vercel's settings or Docker environment) does not automatically overwrite or affect the Gemini key value that might be set on the frontend pages (e.g., if a user manually inputs a key in the application's settings). The frontend has its own mechanism for API key management, and this server-side key is primarily for backend proxying.

Multiple Keys: You can provide multiple Gemini API keys. Each key should be separated by a comma (,). The application might rotate through these keys or use them based on a specific logic to manage rate limits or distribute load.

Format: key1,key2,key3

Example: '''GEMINI_API_KEY=your key'''

# .env.local example for GEMINI_API_KEY
# This is your Google Gemini API key. It is essential for the application to function.
# Obtain it from Google AI Studio: https://aistudio.google.com/app/apikey
# For multiple keys, separate them with commas.
GEMINI_API_KEY="your_single_gemini_api_key_here"
# Or for multiple keys:
# GEMINI_API_KEY="key_part_one,key_part_two,key_part_three"

# Security Best Practice: Never commit your .env.local file to version control.
# It should be ignored by Git (e.g., via a .gitignore entry).
# For deployment platforms like Vercel or Cloudflare, set this in their environment variable settings, not directly in the repo.

GEMINI_API_BASE_URL (optional)
Purpose: This variable allows you to override the default base URL for Gemini API requests. This is particularly useful if you are using a custom proxy server, a regional endpoint, or a service like Cloudflare AI Gateway to route your API calls.

Default Value: If this variable is not set, the application will default to the official Google Gemini API endpoint: https://generativelanguage.googleapis.com.

Examples of Use Cases:

Custom Proxy: http://your-gemini-proxy.com

Regional Endpoint: https://us-east1-aiplatform.googleapis.com (if using Vertex AI, for instance)

Cloudflare AI Gateway: https://gateway.ai.cloudflare.com/v1/ACCOUNT_ID/GATEWAY_NAME/workers-ai/r2r/llm/@cf/google/gemini-pro (example, actual URL varies)

Security Note: Important: To prevent potential server-side proxy URL leakage or client-side tampering, the value set for GEMINI_API_BASE_URL on the server will not overwrite or directly affect the API base URL that might be configured or displayed on the frontend page. This separation ensures that your proxy endpoint remains a server-side concern.

'''# .env.local example for GEMINI_API_BASE_URL'''
'''# This overrides the default Gemini API endpoint. Useful for proxies or regional endpoints.'''
'''# Default is 'https://generativelanguage.googleapis.com'''
'''# GEMINI_API_BASE_URL="http://your-custom-proxy.com/gemini"'''
'''# GEMINI_API_BASE_URLhttps://us-central1-aiplatform.googleapis.com'''

NEXT_PUBLIC_GEMINI_MODEL_LIST (optional)
Purpose: This variable allows you to customize the list of Gemini models that are available for selection within the Vorttertex AI Chat application's user interface. By default, the application aims to include all commonly available Gemini models.

Default Value: all (This means all models accessible via the API are displayed).

Format for Multiple Models: Multiple model names should be separated by commas (,).

Customization Examples:

Show specific models: If you only want gemini-pro and gemini-1.5-flash-latest, you would set:
NEXT_PUBLIC_GEMINI_MODEL_LIST=gemini-pro,gemini-1.5-flash-latest

Add a new model to the default list: To include a newly released model while keeping others:
NEXT_PUBLIC_GEMINI_MODEL_LIST=all,+new-experimental-model

Remove an existing model from the default list: To exclude gemini-pro-vision from the default all list:
NEXT_PUBLIC_GEMINI_MODEL_LIST=all,-gemini-pro-vision

Remove all defaults and define entirely new list: If you want a completely custom list, start with -all:
NEXT_PUBLIC_GEMINI_MODEL_LIST=-all,my-custom-gemini-model,another-special-model

Set a default selected model: You can specify which model should be pre-selected as the default in the UI by using the @ symbol.
NEXT_PUBLIC_GEMINI_MODEL_LIST=all,@gemini-1.5-flash-latest (Sets gemini-1.5-flash-latest as default, but all are available).

# .env.local example for NEXT_PUBLIC_GEMINI_MODEL_LIST
# Customize the visible Gemini model list in the UI.
# Default: 'all'
# Examples:
# NEXT_PUBLIC_GEMINI_MODEL_LIST="gemini-pro,gemini-1.5-flash-latest"
# NEXT_PUBLIC_GEMINI_MODEL_LIST="all,+gemini-advanced"
# NEXT_PUBLIC_GEMINI_MODEL_LIST="all,-gemini-pro-vision"
# NEXT_PUBLIC_GEMINI_MODEL_LIST="all,@gemini-1.5-pro-latest" # Set 1.5 Pro as default

NEXT_PUBLIC_UPLOAD_LIMIT (optional)
Purpose: This variable allows you to set a maximum file size limit for uploads within the application. This is particularly relevant for features involving multi-modal inputs like image or document uploads.

Default Value: By default, there is no explicit file size limit enforced by this variable (though underlying server or API limits might still apply).

Format: Specify the limit using common size units.

Examples:

10MB for 10 megabytes

500KB for 500 kilobytes

2GB for 2 gigabytes

# .env.local example for NEXT_PUBLIC_UPLOAD_LIMIT
# Set the maximum file upload size limit. No limit by default.
# Examples: "10MB", "512KB", "2GB"
# NEXT_PUBLIC_UPLOAD_LIMIT="10MB"

ACCESS_PASSWORD (optional)
Purpose: This variable provides a simple layer of access control for your deployed Vorttertex AI Chat application. If set, users will be required to enter this password before they can access the application's interface.

Security Note: This is a basic access control and should not be considered a robust security solution for highly sensitive environments. For enterprise-grade security, more advanced authentication mechanisms (e.g., OAuth, SSO) would be required. However, it's effective for preventing casual unauthorized access to your personal deployment.

Impact: After setting or modifying this environment variable, you must redeploy the project (on Vercel, Cloudflare, or Docker) for the changes to take effect. If you forget the password, you'll need to update this variable with a new one and redeploy.

# .env.local example for ACCESS_PASSWORD
# Set a password to restrict access to your deployed application.
# If left empty, no password will be required.
# ACCESS_PASSWORD="your_strong_access_password_here"

HEAD_SCRIPTS (optional)
Purpose: This variable allows you to inject custom HTML script tags directly into the <head> section of your application's index.html page. This is primarily useful for integrating third-party services that require script injection, such as analytics tools, error tracking services, or custom web fonts.

Format: Provide the full HTML <script> tags, including opening and closing tags, and any attributes (e.g., src, async, defer).

Examples:

Google Analytics:
HEAD_SCRIPTS=<script async src="https://www.googletagmanager.com/gtag/js?id=G-XXXXX"></script><script>window.dataLayer=window.dataLayer||[];function gtag(){dataLayer.push(arguments);}gtag('js',new Date());gtag('config','G-XXXXX');</script>

Custom Font Loaders:
HEAD_SCRIPTS=<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin><link href="https://fonts.googleapis.com/css2?family=Roboto&display=swap" rel="stylesheet">

# .env.local example for HEAD_SCRIPTS
# Inject custom script code into the <head> section of the HTML.
# Useful for analytics, error tracking, or custom fonts.
# Example for Google Analytics (replace with your actual ID):
# HEAD_SCRIPTS="<script async src='https://www.googletagmanager.com/gtag/js?id=G-123456789'></script><script>window.dataLayer=window.dataLayer||[];function gtag(){dataLayer.push(arguments);}gtag('js',new Date());gtag('config','G-123456789');</script>"

EXPORT_BASE_PATH (optional)
Purpose: This variable is specifically used when deploying Vorttertex AI Chat in static deployment mode (e.g., to GitHub Pages) and when the application is hosted in a subdirectory rather than the root of the domain. It tells the application the base path from which its assets should be loaded.

Use Case: If your deployed application is accessed at https://your-domain.com/my-app/ instead of https://your-domain.com/, you would set EXPORT_BASE_PATH=/my-app. This ensures that all internal links and asset paths (JavaScript bundles, CSS, images) are correctly resolved.

Impact: If you deploy to a subdirectory without setting this, you might experience broken links, missing styles, or non-functional JavaScript because the application will try to load resources from the root (/) instead of the correct subdirectory (/my-app/).

# .env.local example for EXPORT_BASE_PATH
# Only used for static deployment if the project is in a subdirectory (e.g., GitHub Pages).
# Example: If your app is at https://username.github.io/my-chat-app/
# EXPORT_BASE_PATH="/my-chat-app"

Development
Setting up Vorttertex AI Chat for local development is straightforward. This section guides you through the necessary steps to get the project running on your local machine, allowing you to contribute, test features, or customize the application.

If you have not installed pnpm yet, it's the recommended package manager for this project due to its efficiency in managing dependencies.

# Install pnpm globally if you don't have it.
# pnpm is a fast, disk space efficient package manager.
npm install -g pnpm

Once pnpm is installed, you can proceed with setting up the development environment:

# Step 1: Ensure you have Node.js and Yarn (or npm, but pnpm is preferred) installed.
# Node.js version 18 or later is recommended for full compatibility.
# You can check your Node.js version with: node -v
# You can check your pnpm version with: pnpm -v

# Step 2: Configure local variables.
# This is a critical step for local development.
# The project includes an example environment file: `.env.example`
# You need to copy this file and rename it to either `.env` or `.env.local`.
#
#   - `.env`: Used for general environment variables.
#   - `.env.local`: Used for local-only environment variables that should not be committed to Git.
#                   This is the recommended file for sensitive data like API keys.
#
# Open the newly created `.env` or `.env.local` file and populate the `GEMINI_API_KEY`
# with your actual Gemini API key obtained from Google AI Studio.
# You can also set `ACCESS_PASSWORD` if you want to test access control locally.
# Example content for `.env.local`:
# ----------------------------------
# GEMINI_API_KEY="AIzaSyB-Your_Local_Dev_Gemini_API_Key_Here"
# ACCESS_PASSWORD="my_dev_password"
# NEXT_PUBLIC_GEMINI_MODEL_LIST="all"
# ----------------------------------
cp .env.example .env.local # Copy the example to a local environment file

# Step 3: Install project dependencies using pnpm.
# This command reads the `pnpm-lock.yaml` file and installs all required packages.
pnpm install

# Step 4: Run the development server.
# This command starts the Next.js development server, typically on port 3000.
# It enables hot-reloading, so changes to your code will automatically reflect in the browser.
pnpm dev

# After running 'pnpm dev', open your browser and navigate to:
# http://localhost:3000
# You should see the Vorttertex AI Chat application running locally.

Requirements
To ensure a smooth development experience and full compatibility with the project's dependencies and tools, please ensure your development environment meets the following requirements:

NodeJS >= 18: Node.js is the JavaScript runtime environment. Version 18 or later is required due to features utilized by Next.js and other project dependencies. You can download the latest LTS (Long Term Support) version from the official Node.js website.

# Check your Node.js version
node -v
# If it's older than 18, consider updating via nvm (Node Version Manager) or official installer.

Docker >= 20: Docker is essential if you plan to build or run the application using Docker containers. Version 20 or newer is required to ensure compatibility with the Docker commands and features used in the deployment section, particularly for handling docker pull and docker run commands with the specified image.

# Check your Docker version
docker version
# Ensure both Client and Server versions are 20.x or higher.

Rust (latest stable version): Rust is a prerequisite specifically for building the Tauri desktop application. Tauri leverages Rust for its native backend, providing a lightweight and high-performance cross-platform binary. If you're only working on the web application, Rust might not be strictly necessary, but it's vital for desktop builds.

# Install Rust via rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Check Rust version
rustc --version
cargo --version

pnpm: As noted, pnpm is the preferred package manager. It's faster and more efficient with disk space than npm or yarn.

# Install pnpm if you haven't already
npm install -g pnpm
# Check pnpm version
pnpm -v

Ensuring these prerequisites are met will prevent common setup issues and allow for a seamless development and build process.

Deployment
Deploying Vorttertex AI Chat allows you to host your private AI assistant on various platforms, making it accessible from anywhere. This section outlines the recommended deployment methods: Docker for server-based deployments and Static Deployment for flexible web hosting services.

Docker (Recommended)
Docker provides a robust and portable way to deploy Vorttertex AI Chat as a self-contained application. This method is highly recommended for server environments, as it encapsulates all dependencies within a container, ensuring consistent performance across different hosts.

Important Docker Version Note: The Docker daemon and client version need to be 20.0 or above. Older versions might encounter issues, such as the docker pull command prompting that the image cannot be found or other compatibility problems due to changes in Docker's API or registry interactions. Please ensure your Docker installation is up-to-date.

⚠️ Update Lag Notice: It's common for Docker images to lag behind the very latest code changes in the GitHub repository by approximately 1 to 2 days due to build pipeline schedules. Therefore, after deploying with Docker, you might occasionally see an "update exists" prompt within the application even if you've pulled the latest tag. This is normal behavior and indicates that a newer version of the source code is available but has not yet been built into the Docker image.

To deploy using Docker, follow these steps:

Pull the latest Docker image: This command downloads the pre-built Docker image for Vorttertex AI Chat from Docker Hub. The latest tag ensures you get the most recently released stable version.

# Pull the Docker image for talk-with-gemini (the Docker Hub name used by the upstream)
docker pull xiangfa/talk-with-gemini:latest

Run the Docker container: This command starts a new Docker container based on the pulled image.

# Run the Docker container, mapping port 5481 on your host to port 3000 in the container.
docker run -d --name talk-with-gemini -p 5481:3000 xiangfa/talk-with-gemini

Specifying Additional Environment Variables for Docker
You can pass environment variables, such as your Gemini API key and access password, directly to the Docker container when you run it. This is the secure and recommended way to configure sensitive information for Docker deployments.

# Example: Running the Docker container with GEMINI_API_KEY and ACCESS_PASSWORD
docker run -d --name talk-with-gemini \
  -p 5481:3000 \
  -e GEMINI_API_KEY=AIzaSy... \
  -e ACCESS_PASSWORD=your-password \
  xiangfa/talk-with-gemini

Deploy using docker-compose.yml
For more complex deployments or when managing multiple services, using docker-compose is highly recommended. It allows you to define your services, networks, and volumes in a single YAML file, making your deployment configuration declarative and easy to manage.

Create a docker-compose.yml file: In your deployment directory, create a file named docker-compose.yml (or docker-compose.yaml) and paste the following content:

# docker-compose.yml
# This file defines the services, networks, and volumes for your Docker-based deployment.
# It provides a declarative way to manage your containerized application.

version: '3.9' # Specifies the Docker Compose file format version. Version 3.9 is a good modern choice.

services:
  # Defines a service named 'talk-with-gemini' which will run your AI chat application.
  talk-with-gemini:
    image: xiangfa/talk-with-gemini # Specifies the Docker image to use. This will be pulled from Docker Hub.
    container_name: talk-with-gemini # Assigns a specific name to the container for easy identification.
    restart: unless-stopped # Configures the container to automatically restart unless explicitly stopped.
                            # This ensures high availability of your application.
    environment:
      # Environment variables passed into the container.
      # These are crucial for configuring the application.
      - GEMINI_API_KEY=AIzaSy... # Replace 'AIzaSy...' with your actual Gemini API key.
                                 # This is mandatory for the AI functionality.
                                 # For multiple keys, use comma separation: key1,key2
      - ACCESS_PASSWORD=your-password # Replace 'your-password' with your desired access password.
                                      # If left blank, no password will be required.
                                      # Example: ACCESS_PASSWORD=supersecure!
      # Optional environment variables can be added here as well:
      # - GEMINI_API_BASE_URL=http://your-proxy-url.com
      # - NEXT_PUBLIC_GEMINI_MODEL_LIST=gemini-pro,gemini-1.5-flash-latest
      # - NEXT_PUBLIC_UPLOAD_LIMIT=20MB

    ports:
      # Port mappings between the host machine and the container.
      # The format is HOST_PORT:CONTAINER_PORT
      - 5481:3000 # Maps host port 5481 to container port 3000.
                  # You will access the application via http://your_server_ip:5481

Run Docker Compose: Navigate to the directory containing your docker-compose.yml file in your terminal and run:

# Start the services defined in docker-compose.yml in detached mode.
docker compose up -d
# Or for older Docker Compose versions:
# docker-compose up -d

Static Deployment
For scenarios where you prefer to host your application on a static web server or a static site hosting service, Vorttertex AI Chat can be built into a set of static assets. This method offers excellent performance, simplicity, and security, as it only serves pre-generated files.

Build the static page version: This command tells Next.js to export your application as static HTML, CSS, and JavaScript files.

# Execute the command to build and export the static version of the application.
pnpm build:export

Upload files to a static hosting service: Once the pnpm build:export command completes, you will find a new directory named out in your project's root. This out directory contains all the necessary static files (HTML, CSS, JavaScript, images) for your application.
You can then upload all the contents of this out directory to any website service that supports static pages. Popular choices include:

GitHub Pages: Ideal for personal projects, hosted directly from a GitHub repository.

Cloudflare Pages: Offers a global CDN, excellent performance, and easy integration with GitHub.

Vercel (Static Hosting): While Vercel also supports serverless functions, it's highly optimized for static site deployments.

Netlify: Another popular choice for static sites with powerful features.

AWS S3 + CloudFront: For robust, scalable, and highly customizable static hosting on AWS.

Nginx/Apache: If you have your own web server, you can simply serve the contents of the out directory.

Simply drag-and-drop the out folder's contents (not the folder itself) to your hosting provider's upload interface, or configure your CI/CD pipeline to deploy this directory.

<!--
Important considerations for Static Deployment:
- **Root vs. Subdirectory Deployment**:
  - If you deploy to the root of a domain (e.g., `https://your-domain.com/`), no extra configuration is usually needed for paths.
  - If you deploy to a subdirectory (e.g., `https://your-domain.com/my-vorttertex-app/`), you **MUST** set the `EXPORT_BASE_PATH` environment variable during build time. Refer to the `EXPORT_BASE_PATH` environment variable section for details.
- **Client-Side Rendering**: In static deployments, initial data fetching for dynamic content (like AI responses) happens client-side after the HTML loads. This is typically fine for chat applications.
- **No Server-Side Logic**: Static deployments do not run Node.js servers for API routes. If you use a server-side API key (set via `GEMINI_API_KEY` in Vercel/Cloudflare's environment settings), these platforms might still proxy your requests, but the "server" itself is minimal. For pure static hosting, the frontend directly calls the Gemini API or a separate API proxy.
-->

If you deploy the project in a subdirectory (e.g., https://your-domain.com/my-chat/) and encounter resource loading failures (e.g., missing styles, broken images, non-functional JavaScript) when accessing the application, this is usually because the application is trying to load assets from the root path (/) instead of its actual subdirectory. To fix this, you need to configure the base path during the build process. Please add the EXPORT_BASE_PATH environment variable in your .env file (for local builds) or in the variable setting page of your hosting service (e.g., Vercel, Cloudflare Pages, Netlify) before triggering the static build.

For example, if your application is deployed to https://your-domain.com/my-chat/, you would set:
EXPORT_BASE_PATH=/my-chat

Acknowledgments
Vorttertex AI Chat stands on the shoulders of giants, leveraging the power and innovation of various open-source technologies and drawing inspiration from exemplary projects within the AI and web development communities. We extend our sincere gratitude to the creators and maintainers of these foundational components.

Technology Stack
The robust and efficient architecture of Vorttertex AI Chat is built upon a modern and high-performance technology stack, chosen for its scalability, developer experience, and community support:

Next.js: A powerful React framework that enables server-side rendering (SSR), static site generation (SSG), and API routes. Next.js provides the foundational framework for the web application, optimizing for performance, SEO, and developer productivity. It plays a crucial role in providing both the web and the core of the desktop application (via Tauri).

<!--
Key benefits of Next.js for Vorttertex AI Chat:
- **Hybrid Rendering**: Allows for a mix of SSR, SSG, and Client-Side Rendering (CSR), offering flexibility for different parts of the application.
- **API Routes**: Enables the creation of serverless functions within the Next.js project, used for proxying Gemini API calls and handling plugin logic securely.
- **Image Optimization**: Built-in `next/image` component for efficient, responsive image loading, improving visual performance.
- **File-system based routing**: Simplifies page creation and navigation.
- **TypeScript support**: Provides strong typing for more robust and maintainable code.
- **Fast Refresh**: Offers a quick development loop with instant feedback on code changes.
-->

Shadcn UI: A collection of beautifully designed, accessible, and customizable UI components built using Radix UI and Tailwind CSS. Shadcn UI provides the aesthetic and functional building blocks for the user interface, ensuring a consistent, modern, and highly polished look and feel across the application. It streamlines UI development by offering pre-built, composable components.

<!--
Advantages of Shadcn UI:
- **Headless Components (Radix UI)**: Provides low-level, unstyled components that handle accessibility and interaction logic, allowing for full styling flexibility with Tailwind CSS.
- **Tailwind CSS Integration**: Seamlessly integrates with Tailwind CSS for powerful styling capabilities.
- **Copy-and-paste approach**: Components are copied into the project, giving developers full control over customization without being locked into a dependency.
- **Accessibility**: Components are built with accessibility in mind, following WAI-ARIA guidelines.
- **Theming**: Easy to theme and adapt to specific branding or dark/light modes.
-->

Tailwind CSS: A highly customizable, utility-first CSS framework. Tailwind CSS is used for rapidly building custom designs without leaving your HTML (or JSX). Its atomic CSS classes allow for precise styling and efficient component styling, contributing to the responsive and modern design of Vorttertex AI Chat while keeping CSS bundles small.

<!--
Why Tailwind CSS for this project:
- **Utility-First**: Offers low-level utility classes that can be composed to build any design.
- **Customizability**: Extremely configurable, allowing projects to define their design system.
- **Performance**: Purges unused CSS classes in production builds, leading to very small CSS file sizes.
- **Developer Experience**: Speeds up UI development by allowing styling directly in JSX/HTML without context switching to separate CSS files.
- **Consistency**: Encourages a consistent design by reusing utility classes.
-->

Zustand: A small, fast, and scalable bear-necessities state-management solution for React. Zustand is utilized for managing the application's global state, such as conversation history, user settings, and AI model configurations. Its minimalistic API, performant updates, and unopinionated nature make it an excellent choice for handling complex application state efficiently and predictably.

<!--
Benefits of Zustand for state management:
- **Simplicity**: Very small API, easy to learn and use.
- **Speed**: Optimized for fast updates, minimizing re-renders.
- **Scalability**: Suitable for both small and large applications.
- **No boilerplate**: Less code compared to other state management libraries.
- **Hooks-based**: Integrates naturally with React hooks.
- **TypeScript friendly**: Provides excellent TypeScript support for type safety.
-->

Tauri: A framework for building tiny, blazing-fast, and secure cross-platform desktop applications using web technologies. Tauri is central to creating the lightweight desktop client for Vorttertex AI Chat, providing a native shell (built in Rust) for the web-based UI (Next.js). This allows for a much smaller binary size and better performance compared to frameworks like Electron, while still leveraging the existing web codebase.

<!--
Why Tauri for desktop client:
- **Small Footprint**: Generates significantly smaller binaries compared to Electron.
- **Performance**: Leverages Rust for native performance and efficiency.
- **Security**: Focuses on security by design, with features like isolation and sandboxing.
- **Cross-Platform**: Supports Windows, macOS, and Linux from a single codebase.
- **Web Technologies**: Allows developers to use HTML, CSS, and JavaScript/TypeScript for UI.
- **System Tray/Menu Bar Integration**: Enables the client to run in the background with quick access.
-->

Inspiration
Vorttertex AI Chat draws significant inspiration from other pioneering projects in the AI chat and web development space. These projects have set high standards for design, functionality, and user experience, guiding the development of Vorttertex AI Chat towards excellence.

Lobe Chat: A highly aesthetic and feature-rich AI conversational UI. Lobe Chat's elegant design, advanced features (like function calling and agent capabilities), and user-friendly interface have provided valuable insights into creating a polished and intelligent AI chat experience. Its focus on extensibility and modern UI patterns has been a key source of inspiration for Vorttertex AI Chat's design philosophy.

Next Web: An open-source, extensible, high-performance ChatGPT chatbot UI. Next Web's emphasis on performance, customizability, and robust API integration has influenced the architectural decisions in Vorttertex AI Chat, particularly regarding fast loading times, streaming responses, and efficient interaction with AI models. Its commitment to a clean and functional interface is also a notable point of reference.

Open Canvas: While more focused on multimodal AI and agentic workflows, Open Canvas showcases innovative approaches to interactive AI. Its exploration of advanced multimodal interactions and the orchestration of complex AI tasks provides a forward-looking perspective that inspires the continuous development of Vorttertex AI Chat's advanced features, particularly in areas like multimodal live interactions and future research modes.

We are grateful to these projects for their contributions to the open-source community and for providing valuable benchmarks and ideas that have shaped the development of Vorttertex AI Chat.

FAQ
This section addresses frequently asked questions and common issues users might encounter while using or deploying Vorttertex AI Chat. We aim to provide clear and actionable solutions to help you troubleshoot effectively.

Solution for "User location is not supported for the API use"
This error typically indicates that your current geographical location is restricted from accessing the Google Gemini API directly, or that the specific API endpoint you're trying to reach does not support requests from your region. This is a common geopolitical or service-region restriction.

Here are the recommended solutions:

Use Cloudflare AI Gateway to forward APIs (Recommended): Cloudflare AI Gateway acts as an intelligent proxy that can route your API requests through Cloudflare's global network, often bypassing regional restrictions. It provides a reliable and fast way to access AI APIs.

How it works: Your application sends requests to the Cloudflare AI Gateway, which then forwards them to the Google Gemini API from a Cloudflare data center that is permitted to access the service.

Advantages: This solution is generally fast, stable, and secure. It abstracts away the regional complexities.

Setup: For detailed instructions on how to set up and use Cloudflare AI Gateway with Vorttertex AI Chat, please refer to the dedicated documentation: How to Use Cloudflare AI Gateway. This guide provides step-by-step instructions to configure the gateway and update your GEMINI_API_BASE_URL.

<!--
Cloudflare AI Gateway setup outline:
- Sign up for a Cloudflare account.
- Create an AI Gateway.
- Configure the gateway to point to the Google Gemini API.
- Obtain your unique gateway URL (e.g., `https://gateway.ai.cloudflare.com/v1/ACCOUNT_ID/GATEWAY_NAME/workers-ai/r2r/llm/@cf/google/gemini-pro`).
- Set this URL as your `GEMINI_API_BASE_URL` environment variable in your deployment (Vercel, Docker, etc.).
- Ensure your application's API requests are directed to this new base URL.
-->

Use Cloudflare Worker for API proxy forwarding (Alternative): As an alternative to the AI Gateway, you can deploy a custom Cloudflare Worker to act as a proxy for your Gemini API calls. This gives you more granular control over the proxy logic.

How it works: You write a small JavaScript code (a Worker) that runs on Cloudflare's edge network. This Worker intercepts requests from your application, adds your API key securely (if desired), and forwards them to the Gemini API.

Considerations: While powerful, this solution might require more setup and configuration compared to the AI Gateway. There's also a possibility that certain configurations might not work properly in all cases or might be subject to Cloudflare Worker limitations.

Setup: For detailed settings and a step-by-step guide, please refer to: How to Use Cloudflare Worker Proxy API.

<!--
Cloudflare Worker proxy setup outline:
- Create a new Cloudflare Worker script.
- Write JavaScript code within the Worker to:
    - Receive incoming requests (from your Vorttertex AI Chat app).
    - Modify request headers (e.g., add Authorization header with API key).
    - Forward the request to the official Gemini API endpoint.
    - Handle the response from Gemini and send it back to your app.
- Deploy the Worker.
- Configure your Vorttertex AI Chat's `GEMINI_API_BASE_URL` to point to your deployed Worker's URL.
This method provides high customization but requires some coding.
-->

Why can't I access the website in China after deploying it with one click using Vercel?
If you've deployed Vorttertex AI Chat on Vercel and are experiencing issues accessing it from mainland China, this is a known challenge related to network restrictions.

Vercel Domain Blocking: The default .vercel.app domain names generated after deploying projects on Vercel were subject to blocking by China's Great Firewall several years ago. This means that while the underlying server IP addresses might still be accessible, the specific domain name is blocked at the network level, preventing direct access.

Server IP Accessibility: Fortunately, the actual IP addresses of Vercel's servers (which are typically outside China) are generally not blocked. This implies that the issue is primarily with the domain name resolution.

Network Fluctuations: Since Vercel does not have servers located within mainland China, it's normal to experience some network fluctuations, higher latency, or occasional access issues even with a custom domain, due to the inherent challenges of cross-border internet traffic.

Solution: Customize the Domain Name
The most effective solution to this problem is to configure a custom domain name for your Vercel deployment. By binding your own domain (e.g., myvorttertexchat.com) to your Vercel project, you bypass the blocked .vercel.app domain.

How to Set a Custom Domain:
You can refer to general online resources and documentation for binding a custom domain name to a Vercel project. A helpful article I found online (which you can use as a reference, though specifics might vary slightly by domain registrar) is: Vercel binds a custom domain name.
The general steps involve:

Purchasing a domain name from a domain registrar.

Adding the custom domain to your Vercel project settings.

Updating your domain registrar's DNS records (e.g., CNAME or A records) to point to Vercel's servers.

By using a custom domain, your application will typically be accessible normally from within China, leveraging the unblocked IP addresses of Vercel's global network.

Why can't I use Multimodal Live?
If you are facing issues using the Multimodal Live API feature in Vorttertex AI Chat, there are a few key points to consider:

Model Requirement: Gemini 2.0 Flash: The Multimodal Live API is a cutting-edge feature that is currently only supported by the Gemini 2.0 Flash model. Ensure that you have selected gemini-1.5-flash-latest (or an equivalent Gemini 2.0 Flash model version) as your active model within the Vorttertex AI Chat settings. If you are using an older Gemini Pro or Gemini Pro Vision model, Multimodal Live functionality will not be available. You might need to update your NEXT_PUBLIC_GEMINI_MODEL_LIST environment variable to ensure this model is visible and selectable in your UI.

<!--
To ensure Gemini 2.0 Flash is available and selected:
1. Verify `NEXT_PUBLIC_GEMINI_MODEL_LIST` includes `gemini-1.5-flash-latest` (or `all`).
   Example: `NEXT_PUBLIC_GEMINI_MODEL_LIST="all,@gemini-1.5-flash-latest"`
2. In the Vorttertex AI Chat application, go to settings and explicitly select `Gemini 1.5 Flash` as your model.
-->

Access Restrictions in China: The Google Gemini Multimodal Live API, like other Google services, may not be directly accessible within mainland China due to network restrictions. If you are deploying or accessing the application from China, direct API calls to generativelanguage.googleapis.com for the Multimodal Live API will likely fail.

Solution: Proxying the Multimodal Live API with Cloudflare Worker:
To overcome regional access restrictions for the Multimodal Live API, you will likely need to deploy a proxy. A Cloudflare Worker is an excellent solution for this.

How it works: A Cloudflare Worker can intercept the API calls from your Vorttertex AI Chat application, proxy them through Cloudflare's global network (which often bypasses regional blocks), and then forward them to the actual Gemini Multimodal Live API endpoint.

Setup: For a detailed guide on how to configure and deploy a Cloudflare Worker specifically for proxying the Multimodal Live API, please refer to: Proxying the Multimodal Live API with Cloudflare Worker. This document will provide the necessary code and configuration steps.

<!--
Brief technical explanation of proxying Multimodal Live API:
- The Multimodal Live API might use different endpoints or protocols (e.g., WebSockets for streaming) than regular text APIs.
- A Cloudflare Worker can be configured to handle these specific streaming requests.
- The Worker acts as an intermediary, receiving the stream from the client, forwarding it to Gemini, and then streaming the response back.
- This circumvents network blocks by routing traffic through Cloudflare's edge locations.
-->

Chinese Voice Output Limitation:
Currently, the Multimodal Live API does not support Chinese voice output. While the API may support Chinese voice input (Speech-to-Text), the Text-to-Speech (TTS) component for generating spoken responses in Chinese is not yet available through this specific API. This means if you expect the AI to speak back to you in Chinese, this feature will not function as desired for that language. Please be aware of this limitation when using Multimodal Live for Chinese conversations.

Contributing
We welcome and appreciate contributions to Vorttertex AI Chat! Whether you're fixing a bug, adding a new feature, or improving documentation, your efforts help make this project better for everyone. Please follow these guidelines to ensure a smooth and effective contribution process.

Fork the repository on GitHub: Start by forking the official Adevloper152/Vorttertex-ai-chat repository to your personal GitHub account. This creates a copy of the project under your username where you can make changes without affecting the original codebase directly.

# Example: Fork the repository from the GitHub UI, then proceed with cloning your fork.

Clone your fork to your local machine: After forking, clone your personal copy of the repository to your local development environment. This allows you to work on the code using your preferred editor and tools.

# Replace 'your-username' with your actual GitHub username.
git clone https://github.com/your-username/Vorttertex-ai-chat.git
cd Vorttertex-ai-chat

Create a new branch for your changes: Before making any modifications, always create a new branch. This keeps your changes isolated, making it easier to manage different features or bug fixes and facilitating the pull request process. Choose a descriptive branch name (e.g., feature/add-weather-plugin, bugfix/fix-chat-scroll).

# Example: Create and switch to a new branch for your work.
git checkout -b feature/your-awesome-feature-name

Make your changes and commit them to your branch: Implement your desired features, bug fixes, or improvements. Write clean, readable code that adheres to the project's coding style (if any specific style guides are provided). After making changes, commit them with clear and concise commit messages that summarize your modifications.

# Example: Add all changed files to the staging area.
git add .
# Example: Commit staged changes with a descriptive message.
git commit -m "feat: Add new awesome feature to the chat interface"
# Or for a bug fix:
# git commit -m "fix: Resolve issue with multimodal live API connection"

Push your changes to your fork on GitHub: Once you've committed your changes locally, push your new branch and its commits to your personal fork on GitHub.

# Push your current branch to your fork.
git push origin feature/your-awesome-feature-name

Open a pull request from your branch to the main repository: After pushing your changes to your fork, navigate to the original Adevloper152/Vorttertex-ai-chat repository on GitHub. You should see a prompt to open a new pull request from your recently pushed branch.

Fill out the Pull Request (PR) template: Provide a clear title and detailed description of your changes. Explain what problem your PR solves, how you solved it, and any relevant context.

Reference issues: If your PR addresses an existing issue, link it using keywords like Closes #XYZ or Fixes #XYZ.

Screenshots/Videos: Include screenshots or short video demonstrations for UI changes or new features.

<!--
Tips for a successful Pull Request:
- **Clear Title**: Summarize the PR's purpose concisely.
- **Detailed Description**: Explain the changes thoroughly.
- **Motivation**: Why was this change needed? What problem does it solve?
- **How it was implemented**: Briefly describe the technical approach.
- **Testing**: How did you test your changes? (e.g., "Tested locally with `pnpm dev`," "Verified on Vercel deployment").
- **Follow the PR template**: Many repositories have a predefined PR template; fill it out completely.
- **Be responsive**: Be prepared to respond to feedback from maintainers and make requested adjustments.
- **Squash commits (optional but good practice)**: Before merging, you might be asked to squash your commits into a single, clean commit for a cleaner history.
-->

Please ensure that your code follows the project's coding style (if any specific guidelines are established in a CONTRIBUTING.md file) and that all automated tests (if present) pass before submitting a pull request. Running pnpm lint and pnpm test (if available) locally can help catch issues early. If you find any bugs or have suggestions for improvements that are too large for a direct PR, feel free to open an issue on GitHub first to discuss your ideas. Your contributions are highly valued!

LICENSE
This project is licensed under the MIT License. This is a permissive open-source license that allows you to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the software.

<!--
Key aspects of the MIT License:
- **Freedom of Use**: You can use the software for any purpose, including commercial applications.
- **Freedom to Modify**: You can modify the software as you see fit.
- **Freedom to Distribute**: You can distribute the original or modified versions of the software.
- **No Warranty**: The software is provided "as is," without any warranty, express or implied. The authors are not liable for any damages arising from its use.
- **Attribution**: You must include the original copyright and license notice in all copies or substantial portions of the software.

See the [LICENSE](https://www.apache.org/licenses/LICENSE-2.0) file for the full license text.
This ensures transparency and defines the legal terms under which the software is provided.
-->

Vorttertex AI Chat (Desktop Client)
This section focuses specifically on the desktop client application for Vorttertex AI Chat, which provides a native, high-performance experience by leveraging Tauri. This client extends the web application's functionality to your desktop, offering seamless integration with your operating system.

A powerful AI chat application built with Next.js and Tauri. This combination ensures that the desktop client is not only visually appealing and functionally rich but also remarkably lightweight and efficient, providing a superior user experience compared to typical web-wrapped desktop applications.

Prerequisites
To build the Vorttertex AI Chat desktop client from source, you'll need to set up your development environment with specific tools and libraries. These prerequisites ensure that the Tauri framework can compile and package the application correctly for your target operating system.

For all platforms (Windows, macOS, Linux):
Node.js 16 or later: The core JavaScript runtime for the Next.js frontend. Ensure you have an up-to-date version installed.

# Check Node.js version
node -v
# Recommended to use Node Version Manager (nvm) for easy version management.

Rust (latest stable version): Tauri's backend is written in Rust. You'll need the Rust toolchain installed. The rustup installer is the recommended way to get Rust.

# Install rustup (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Verify Rust installation
rustc --version
cargo --version

pnpm: The preferred package manager for managing JavaScript dependencies.

# Install pnpm globally
npm install -g pnpm
# Verify pnpm installation
pnpm -v

For Windows builds:
To build the desktop client on a Windows machine, you need specific Visual Studio components.

Visual Studio 2019 or later: Download and install Visual Studio (Community Edition is free for individuals). During installation, make sure to select the "Desktop development with C++" workload. This workload includes the necessary C++ build tools and SDKs required by Rust and Tauri.

Windows 10 SDK: This is typically included with the "Desktop development with C++" workload, but ensure it's installed. It provides libraries and headers for Windows API interactions.

<!--
Detailed steps for Windows Prerequisites:
1. **Download Visual Studio Installer**: Go to the official Microsoft Visual Studio website.
2. **Run Installer**: Choose the "Community" version if you are an individual developer.
3. **Select Workloads**: In the Visual Studio Installer, select the "Workloads" tab.
   - Check the box for "**Desktop development with C++**".
   - On the right-hand side, in the "Installation details" pane, ensure that "MSVC v14x - VS 2019 C++ x64/x86 build tools" (or relevant version) and "Windows 10 SDK (latest version)" are selected.
4. **Install**: Proceed with the installation. This might take some time depending on your internet speed.
These components provide the C++ compiler and linker that Rust and Tauri rely on for building the native Windows executable.
-->

For Linux builds:
Building on Linux requires several development libraries. The exact commands might vary slightly depending on your specific Linux distribution, but the following covers common Debian/Ubuntu and Fedora systems.

For Ubuntu/Debian based systems:

sudo apt update # Update package lists
sudo apt install \
    libwebkit2gtk-4.0-dev \ # WebKitGTK development files (essential for Tauri WebView)
    build-essential \       # Meta-package for essential build tools (gcc, g++, make, etc.)
    curl \                  # Tool for transferring data with URLs (used by rustup)
    wget \                  # Non-interactive network downloader (alternative to curl)
    libssl-dev \            # Development files for OpenSSL (for secure communication)
    libgtk-3-dev \          # Development files for GTK+ 3.x graphical toolkit
    libayatana-appindicator3-dev \ # Development files for Ayatana Application Indicators (for tray icon)
    librsvg2-dev            # Library for rendering SVG images (often needed for icons)

For Fedora based systems:

sudo dnf install \
    webkit2gtk3-devel \       # WebKitGTK development files for Fedora
    gtk3-devel \              # GTK+ 3 development files for Fedora
    libappindicator-gtk3-devel \ # AppIndicator development files for Fedora
    librsvg2-devel \          # SVG rendering library development files
    gcc-c++ \                 # C++ compiler (part of build essentials)
    libssl-devel \            # OpenSSL development files
    curl \                    # Curl utility
    wget                      # Wget utility

For macOS builds:
Building on macOS requires Apple's developer tools.

Xcode Command Line Tools: These tools include the necessary compilers (Clang), Git, and other command-line utilities required for building software on macOS. You can install them by running a single command in your terminal.

xcode-select --install

macOS 10.15 (Catalina) or later: Ensure your macOS operating system version is 10.15 or newer for compatibility with Tauri and its dependencies.

<!--
Verification after installing Xcode Command Line Tools:
- You can check if `git` is installed: `git --version`
- You can check if `clang` (the C compiler) is installed: `clang --version`
These tools are fundamental for Rust and Tauri's compilation process on macOS.
-->

Building
Once you have satisfied all the prerequisites for your operating system, you can proceed with building the Vorttertex AI Chat desktop client. This process will compile the Rust backend and package the Next.js frontend into a native executable.

Install dependencies: First, ensure all JavaScript project dependencies are installed. This step is common for both web and desktop builds.

# Navigate to the root directory of your project
# Install all Node.js dependencies as specified in package.json and pnpm-lock.yaml.
pnpm install

Build for your platform: After installing dependencies, you can trigger the build process for your specific operating system. Tauri will handle compiling the Rust backend and bundling the Next.js web application into a native executable.

For macOS:

# Build the application for macOS. This generates a .app bundle.
pnpm tauri:build:mac

For Linux:

# Build the application for Linux. This generates a .deb, .AppImage, or other package.
pnpm tauri:build:linux

For Windows:

# Build the application for Windows. This generates an .exe installer or portable .exe.
pnpm tauri:build:windows

Upon successful completion of the build process, the executable or installer package for Vorttertex AI Chat will be found in the src-tauri/target/release/bundle/ directory (or similar, depending on the platform and build configuration). These packages are ready for distribution or local installation.

Development
If you're interested in actively developing or debugging the Vorttertex AI Chat desktop client, you can run it in a development mode. This mode provides features like live reloading and developer tools for easier iteration.

To run in development mode:

# This command starts both the Next.js development server and the Tauri development environment.
# Changes to your frontend code will typically hot-reload within the Tauri window.
pnpm dev

This command will typically open a new desktop window running the Vorttertex AI Chat application. Any changes you make to the source code (both frontend and backend Rust code, if applicable) will trigger a rebuild and refresh, providing a fast development loop.

License
The Vorttertex AI Chat project, including its desktop client, is released under the MIT License. This ensures that the software is free to use, modify, and distribute, fostering an open and collaborative environment for development and innovation.

<!--
For the full legal text of the MIT License, please refer to the LICENSE file located in the root of this repository.
The MIT License is one of the most permissive open-source licenses, making it easy for others to adopt, adapt, and build upon this project.
-->
# Vortex-ai-chat
# Vortex-ai-chat
