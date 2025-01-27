enum EnumPropEnumeration {
    Unexpected
    Expected
}

[DscResource()]
class TestClassResource
{
    [DscProperty(Key)]
    [string] $Name

    [DscProperty()]
    [string] $Prop1

    [DscProperty()]
    [EnumPropEnumeration] $EnumProp

    [void] Set()
    {
    }

    [bool] Test()
    {
        if (($this.Name -eq "TestClassResource1") -and ($this.Prop1 -eq "ValueForProp1"))
        {
            return $true
        }
        else
        {
            return $false
        }
    }

    [TestClassResource] Get()
    {
        if ($this.Name -eq "TestClassResource1")
        {
            $this.Prop1 = "ValueForProp1"
        }
        $this.EnumProp = [EnumPropEnumeration]::Expected
        return $this
    }
}

function Hello-World()
{
    "Hello world from PSTestModule!"
}
