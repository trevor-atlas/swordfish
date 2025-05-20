// import { TextField } from './fields/TextField'
// import { NumberField } from './fields/NumberField'
// import { SelectField } from './fields/SelectField'
// import { MultiSelectField } from './fields/MultiSelectField'
// import { ConfirmField } from './fields/ConfirmField'
// import { FileField } from './fields/FileField'
// import { DirectoryField } from './fields/DirectoryField'
// import { DateField } from './fields/DateField'
import { Field } from './types'

export function FieldInput({
  field,
  value,
  onChange,
  error,
}: {
  field: Field
  value: any
  onChange: (value: any) => void
  error?: string
}) {
  return null;
  // const commonProps = { field, value, onChange, error }

  // switch (field.type) {
  //   case 'text':
  //   case 'password':
  //     return <TextField {...commonProps} />
  //   case 'number':
  //     return <NumberField {...commonProps} />
  //   case 'select':
  //   case 'radio':
  //     return <SelectField {...commonProps} />
  //   case 'multiselect':
  //     return <MultiSelectField {...commonProps} />
  //   case 'confirm':
  //     return <ConfirmField {...commonProps} />
  //   case 'file':
  //     return <FileField {...commonProps} />
  //   case 'directory':
  //     return <DirectoryField {...commonProps} />
  //   case 'date':
  //   case 'datetime':
  //     return <DateField {...commonProps} />
  //   default:
  //     return <div>Unsupported field type: {field.type}</div>
  // }
}
