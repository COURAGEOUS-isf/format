#let schema = json("schema/courageous.schema.json")

#let describe_type(obj, type_name, level: 2) = [
    #if not obj.keys().contains("type") { return; }
    #heading(type_name, level: level)
    #raw(obj.type)
    #if obj.keys().contains("required") {
        [
            #text("Required members:")
            #enum(..obj.required)
        ]
    }
    #if obj.keys().contains("definitions") {
        for (key, val) in obj.definitions {
            describe_type(val, key, level: level + 1)
        }
    }
]

#describe_type(schema, schema.title)