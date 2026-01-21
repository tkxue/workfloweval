
const worker = new Worker('rescript_worker.js');

const rescriptCode = `
@react.component
let make = () => {
  <div> {React.string("Hello from Worker")} </div>
}
`;

// Send code to worker
worker.postMessage({ code: rescriptCode });

// Listen for the compiled result
worker.onmessage = function(e) {
  const { type, js, errors } = e.data;

  if (type === 'success') {
    console.log('Compiled JS:', js);
  } else {
    console.error('Compilation failed:', errors);
  }
};
