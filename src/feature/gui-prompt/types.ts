export interface BaseField<T = any> {
  type: FieldType
  name: string
  label: string
  default?: T
  hint?: string
  required?: boolean
  validation?: ValidationRule<T> // inline or reference to named validator
  conditional?: ConditionalRule // show/hide logic based on prior answers
}

export interface TextField extends BaseField<string> {
  type: 'text' | 'password'
  multiline?: boolean
}

export interface NumberField extends BaseField<number> {
  type: 'number'
  min?: number
  max?: number
}

export interface SelectField extends BaseField<string> {
  type: 'select' | 'radio'
  options: Option[]
}

export interface MultiSelectField extends BaseField<string[]> {
  type: 'multiselect'
  options: Option[]
}

export interface ConfirmField extends BaseField<boolean> {
  type: 'confirm'
}

export interface DateField extends BaseField<string> {
  type: 'date' | 'datetime'
}

export interface FileField extends BaseField<string> {
  type: 'file'
  accept?: string[] // e.g. ['.jpg', '.png']
  initialPath?: string
}

export interface FilesField extends BaseField<string[]> {
  type: 'files'
  accept?: string[] // e.g. ['.jpg', '.png']
  initialPath?: string
}

export interface DirectoryField extends BaseField<string> {
  type: 'directory'
  initialPath?: string
}

export type Field =
  | TextField
  | NumberField
  | SelectField
  | MultiSelectField
  | ConfirmField
  | DateField
  | FileField
  | FilesField
  | DirectoryField

export type FieldType =
  | 'text'
  | 'password'
  | 'number'
  | 'select'
  | 'multiselect'
  | 'radio'
  | 'confirm'
  | 'date'
  | 'datetime'
  | 'file'
  | 'files'
  | 'directory'

export type Icon = 'image' | 'archive' | 'pdf' | 'file' | 'directory' | (string & {});

export type Option = {
  label: string
  value: string
  icon?: Icon
}

// undefined = valid, string = error message
export type ValidationRule<T> = (input: T) => string | undefined

export type ConditionalRule =
  | {
      dependsOn: string
      condition: (value: any) => boolean
    }

export type Prompt = (form: {
  fields: Record<string, Field>;
  validate: (values: Record<string, string | number | boolean | undefined>) => Promise<Record<string, string | number | boolean | undefined>>;
  onSubmit: (values: Record<string, string | number | boolean | undefined>) => void;
}) => Promise<Record<string, string | number | boolean | undefined>>
