interface InputStream {}
type Token = string
type BytecodeStream = string

abstract class Scanner {
  private _inputStream: InputStream

  constructor(inputStream: InputStream) {}

  abstract scan(): Token
}

abstract class Parser {
  constructor() {}

  abstract parse(scanner: Scanner, builder: ProgramNodeBuilder): void
}

abstract class ProgramNodeBuilder {
  private _node: ProgramNode

  constructor() {}

  abstract newVariable(variableName: string): ProgramNode
  abstract newAssignment(variable: ProgramNode, expression: ProgramNode): ProgramNode
  abstract newReturnStatement(value: ProgramNode): ProgramNode
  abstract newCondition(
    condition: ProgramNode, 
    truePart: ProgramNode, 
    falsePart: ProgramNode
  ): ProgramNode
  abstract getRootNode(): ProgramNode
}

abstract class ProgramNode {
  protected constructor() {}

  abstract getSourcePosition(): { line: number, index: number }
  abstract add(node: ProgramNode): void
  abstract remove(node: ProgramNode): void
  abstract traverse(generator: CodeGenerator): void
}

abstract class CodeGenerator {
  protected _output: BytecodeStream

  constructor(stream: BytecodeStream) {}

  abstract visit(node: ProgramNode): void
}

abstract class Compiler {
  constructor() {}

  abstract compile(inputStream: InputStream, outputStream: BytecodeStream): void
}