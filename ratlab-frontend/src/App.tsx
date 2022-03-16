import { ReactReplView } from 'awesome-react-repl';
import { Ratlab } from 'ratlab';
import { useRef, useState } from 'react';

function App() {
  const [lines, setLines] = useState<any[]>([]);
  const ratlab = useRef(Ratlab.new()).current;

  function handleSubmit(input: string) {
    setLines(lines => lines.concat({ type: 'input', value: input }));

    try {
      const output = ratlab.input(input);
      setLines(lines => lines.concat({ type: 'output', value: output }));
    } catch (e: any) {
      setLines(lines => lines.concat({ type: 'error', value: e.toString() }));
    }
  }

  return (
    <ReactReplView
      title="Ratlab"
      lines={lines}
      onSubmit={handleSubmit}
      height={window.innerHeight}
    />
  );
}

export default App;
