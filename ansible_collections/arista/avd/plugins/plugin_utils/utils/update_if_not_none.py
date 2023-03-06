def update_if_not_none(obj: dict, key: str, value):
    """
    Set key on given dict if value is not None
    """
    if value is not None:
        obj[key] = value
