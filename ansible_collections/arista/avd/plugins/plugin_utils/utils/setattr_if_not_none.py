def setattr_if_not_none(obj: object, attr: str, value) -> None:
    """
    Set attribute on given object if value is not None
    """
    if value is not None:
        setattr(obj, attr, value)
