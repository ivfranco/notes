declare module 'node-plantuml' {
  import stream from 'stream';

  interface Options {
    format: 'png' | 'svg';
  }

  // https://github.com/markushedvall/node-plantuml/blob/e4ee91aab31d3dc19056ec993926df3b153a4839/lib/node-plantuml.js#L139
  interface Generator {
    // https://github.com/markushedvall/node-nailgun-client/blob/2ee278db3dbf2746b4cfb3cdd453e8d78ec4b510/lib/node-nailgun-client.js#L51
    out: stream.Transform;
  }

  // https://github.com/markushedvall/node-plantuml/blob/e4ee91aab31d3dc19056ec993926df3b153a4839/lib/node-plantuml.js#L143
  export function generate(uml: string, options: Options): Generator;
}
