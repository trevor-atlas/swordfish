function useGPT() {
  return [
    { from: 'me', text: 'Hello' },
    { from: 'them', text: 'Hi there, how can I help?' },
  ];
}

export function Chat() {
  const messages = useGPT();

  return (
    <div className="messages-container flex flex-col mt-5">
      {messages.map((message) =>
        message.from === 'me' ? (
          <div className={`flex justify-end mb-4 message ${message.from}`}>
            <div className="mr-2 py-3 px-4 bg-blue-400 rounded-bl-3xl rounded-tl-3xl rounded-tr-xl text-white">
              {message.text}
            </div>
          </div>
        ) : (
          <div className={` flex justify-start mb-4 message ${message.from}`}>
            <div className="ml-2 py-3 px-4 bg-neutral-700 rounded-br-3xl rounded-tr-3xl rounded-tl-xl text-white">
              {message.text}
            </div>
          </div>
        ),
      )}
    </div>
  );
}
