def add(colors: list[list[str]], color: list[str], *, start: int=1, end: int=5):
    colors.append(color)
    for i in range(start,end):
        colors.append(color + [str(i)])

def get_colors() -> list[list[str]]:
    colors = []

    colors.append(["alice", "blue"])
    add(colors,["antique", "white"])
    colors.append(["aqua"])
    add(colors,["aquamarine"])
    add(colors,["azure"])
    colors.append(["beige"])
    add(colors,["bisque"])
    colors.append(["black"])
    colors.append(["blanched", "almond"])
    add(colors,["blue"])
    colors.append(["blue", "violet"])
    add(colors,["brown"])
    add(colors,["burly", "wood"])
    add(colors,["cadet", "blue"])
    add(colors,["chart", "reuse"])
    add(colors,["chocolate"])
    add(colors,["coral"])
    colors.append(["cornflower", "blue"])
    add(colors,["corn", "silk"])
    colors.append(["crimson"])
    add(colors,["cyan"])
    colors.append(["dark", "blue"])
    colors.append(["dark", "cyan"])
    add(colors,["dark", "golden", "rod"])
    colors.append(["dark", "gray"])
    colors.append(["dark", "green"])
    colors.append(["dark", "grey"])
    colors.append(["dark", "khaki"])
    colors.append(["dark", "magenta"])
    add(colors,["dark", "olive", "green"])
    add(colors,["dark", "orange"])
    add(colors,["dark", "orchid"])
    colors.append(["dark", "red"])
    colors.append(["dark", "salmon"])
    add(colors,["dark", "sea", "green"])
    colors.append(["dark", "slate", "blue"])
    add(colors,["dark", "slate", "gray"])
    colors.append(["dark", "slate", "grey"])
    colors.append(["dark", "turquoise"])
    colors.append(["dark", "violet"])
    add(colors,["deep", "pink"])
    add(colors,["deep", "sky", "blue"])
    colors.append(["dim", "gray"])
    colors.append(["dim", "grey"])
    add(colors,["dodger", "blue"])
    add(colors,["fire", "brick"])
    colors.append(["floral", "white"])
    colors.append(["forest", "green"])
    colors.append(["fuchsia"])
    colors.append(["gainsboro"])
    colors.append(["ghost", "white"])
    add(colors,["gold"])
    add(colors,["golden", "rod"])
    add(colors,["gray"], start=0, end=101)
    add(colors,["green"])
    colors.append(["green", "yellow"])
    add(colors,["grey"], start=0, end=101)
    add(colors,["honey", "dew"])
    add(colors,["hot", "pink"])
    add(colors,["indian", "red"])
    colors.append(["indigo"])
    colors.append(["invis"])
    add(colors,["ivory"])
    add(colors,["khaki"])
    colors.append(["lavender"])
    add(colors, ["lavender", "blush"])
    colors.append(["lawn", "green"])
    add(colors,["lemon", "chiffon"])
    add(colors,["light", "blue"])
    colors.append(["light", "coral"])
    add(colors,["light", "cyan"])
    add(colors,["light", "golden", "rod"])
    colors.append(["light", "golden", "rod", "yellow"])
    colors.append(["light", "gray"])
    colors.append(["light", "green"])
    colors.append(["light", "grey"])
    add(colors,["light", "pink"])
    add(colors,["light", "salmon"])
    colors.append(["light", "sea", "green"])
    add(colors,["light", "sky", "blue"])
    colors.append(["light", "slate", "blue"])
    colors.append(["light", "slate", "gray"])
    colors.append(["light", "slate", "grey"])
    add(colors,["light", "steel", "blue"])
    add(colors,["light", "yellow"])
    colors.append(["lime"])
    colors.append(["lime", "green"])
    colors.append(["linen"])
    add(colors,["magenta"])
    add(colors,["maroon"])
    colors.append(["medium", "aquamarine"])
    colors.append(["medium", "blue"])
    add(colors,["medium", "orchid"])
    add(colors,["medium", "purple"])
    colors.append(["medium", "sea", "green"])
    colors.append(["medium", "slate", "blue"])
    colors.append(["medium", "spring", "green"])
    colors.append(["medium", "turquoise"])
    colors.append(["medium", "violet", "red"])
    colors.append(["midnight", "blue"])
    colors.append(["mint", "cream"])
    add(colors,["misty", "rose"])
    colors.append(["moccasin"])
    add(colors,["navajo", "white"])
    colors.append(["navy"])
    colors.append(["navy", "blue"])
    colors.append(["none"])
    colors.append(["old", "lace"])
    colors.append(["olive"])
    add(colors,["olive", "drab"])
    add(colors,["orange"])
    add(colors,["orange", "red"])
    add(colors,["orchid"])
    colors.append(["pale", "golden", "rod"])
    add(colors,["pale", "green"])
    add(colors,["pale", "turquoise"])
    add(colors,["pale", "violet", "red"])
    colors.append(["papaya", "whip"])
    add(colors,["peach", "puff"])
    colors.append(["peru"])
    add(colors,["pink"])
    add(colors,["plum"])
    colors.append(["powder", "blue"])
    add(colors,["purple"])
    colors.append(["rebecca", "purple"])
    add(colors,["red"])
    add(colors,["rosy", "brown"])
    add(colors,["royal", "blue"])
    colors.append(["saddle", "brown"])
    add(colors,["salmon"])
    colors.append(["sandy", "brown"])
    add(colors,["sea", "green"])
    add(colors,["sea", "shell"])
    add(colors,["sienna"])
    colors.append(["silver"])
    add(colors,["sky", "blue"])
    add(colors,["slate", "blue"])
    add(colors,["slate", "gray"])
    colors.append(["slate", "grey"])
    add(colors,["snow"])
    add(colors,["spring", "green"])
    add(colors,["steel", "blue"])
    add(colors,["tan"])
    colors.append(["teal"])
    add(colors,["thistle"])
    add(colors,["tomato"])
    colors.append(["transparent"])
    add(colors,["turquoise"])
    colors.append(["violet"])
    add(colors,["violet", "red"])
    colors.append(["web", "gray"])
    colors.append(["web", "green"])
    colors.append(["web", "grey"])
    colors.append(["web", "maroon"])
    colors.append(["web", "purple"])
    add(colors,["wheat"])
    colors.append(["white"])
    colors.append(["white", "smoke"])
    colors.append(["x11", "gray"])
    colors.append(["x11", "green"])
    colors.append(["x11", "grey"])
    colors.append(["x11", "maroon"])
    colors.append(["x11", "purple"])
    add(colors,["yellow"])
    colors.append(["yellow", "green"])

    return colors

def compute_colors():
    colors = get_colors()
    enum_colors = ["".join(map(lambda x: x.capitalize(), color)) for color in colors]
    lowercase_colors = ["".join(color) for color in colors]
    check_colors = set("""aliceblue 	antiquewhite 	antiquewhite1 	antiquewhite2 	antiquewhite3
    antiquewhite4 	aqua 	aquamarine 	aquamarine1 	aquamarine2
    aquamarine3 	aquamarine4 	azure 	azure1 	azure2
    azure3 	azure4 	beige 	bisque 	bisque1
    bisque2 	bisque3 	bisque4 	black 	blanchedalmond
    blue 	blue1 	blue2 	blue3 	blue4
    blueviolet 	brown 	brown1 	brown2 	brown3
    brown4 	burlywood 	burlywood1 	burlywood2 	burlywood3
    burlywood4 	cadetblue 	cadetblue1 	cadetblue2 	cadetblue3
    cadetblue4 	chartreuse 	chartreuse1 	chartreuse2 	chartreuse3
    chartreuse4 	chocolate 	chocolate1 	chocolate2 	chocolate3
    chocolate4 	coral 	coral1 	coral2 	coral3
    coral4 	cornflowerblue 	cornsilk 	cornsilk1 	cornsilk2
    cornsilk3 	cornsilk4 	crimson 	cyan 	cyan1
    cyan2 	cyan3 	cyan4 	darkblue 	darkcyan
    darkgoldenrod 	darkgoldenrod1 	darkgoldenrod2 	darkgoldenrod3 	darkgoldenrod4
    darkgray 	darkgreen 	darkgrey 	darkkhaki 	darkmagenta
    darkolivegreen 	darkolivegreen1 	darkolivegreen2 	darkolivegreen3 	darkolivegreen4
    darkorange 	darkorange1 	darkorange2 	darkorange3 	darkorange4
    darkorchid 	darkorchid1 	darkorchid2 	darkorchid3 	darkorchid4
    darkred 	darksalmon 	darkseagreen 	darkseagreen1 	darkseagreen2
    darkseagreen3 	darkseagreen4 	darkslateblue 	darkslategray 	darkslategray1
    darkslategray2 	darkslategray3 	darkslategray4 	darkslategrey 	darkturquoise
    darkviolet 	deeppink 	deeppink1 	deeppink2 	deeppink3
    deeppink4 	deepskyblue 	deepskyblue1 	deepskyblue2 	deepskyblue3
    deepskyblue4 	dimgray 	dimgrey 	dodgerblue 	dodgerblue1
    dodgerblue2 	dodgerblue3 	dodgerblue4 	firebrick 	firebrick1
    firebrick2 	firebrick3 	firebrick4 	floralwhite 	forestgreen
    fuchsia 	gainsboro 	ghostwhite 	gold 	gold1
    gold2 	gold3 	gold4 	goldenrod 	goldenrod1
    goldenrod2 	goldenrod3 	goldenrod4 	gray 	gray0
    gray1 	gray10 	gray100 	gray11 	gray12
    gray13 	gray14 	gray15 	gray16 	gray17
    gray18 	gray19 	gray2 	gray20 	gray21
    gray22 	gray23 	gray24 	gray25 	gray26
    gray27 	gray28 	gray29 	gray3 	gray30
    gray31 	gray32 	gray33 	gray34 	gray35
    gray36 	gray37 	gray38 	gray39 	gray4
    gray40 	gray41 	gray42 	gray43 	gray44
    gray45 	gray46 	gray47 	gray48 	gray49
    gray5 	gray50 	gray51 	gray52 	gray53
    gray54 	gray55 	gray56 	gray57 	gray58
    gray59 	gray6 	gray60 	gray61 	gray62
    gray63 	gray64 	gray65 	gray66 	gray67
    gray68 	gray69 	gray7 	gray70 	gray71
    gray72 	gray73 	gray74 	gray75 	gray76
    gray77 	gray78 	gray79 	gray8 	gray80
    gray81 	gray82 	gray83 	gray84 	gray85
    gray86 	gray87 	gray88 	gray89 	gray9
    gray90 	gray91 	gray92 	gray93 	gray94
    gray95 	gray96 	gray97 	gray98 	gray99
    green 	green1 	green2 	green3 	green4
    greenyellow 	grey 	grey0 	grey1 	grey10
    grey100 	grey11 	grey12 	grey13 	grey14
    grey15 	grey16 	grey17 	grey18 	grey19
    grey2 	grey20 	grey21 	grey22 	grey23
    grey24 	grey25 	grey26 	grey27 	grey28
    grey29 	grey3 	grey30 	grey31 	grey32
    grey33 	grey34 	grey35 	grey36 	grey37
    grey38 	grey39 	grey4 	grey40 	grey41
    grey42 	grey43 	grey44 	grey45 	grey46
    grey47 	grey48 	grey49 	grey5 	grey50
    grey51 	grey52 	grey53 	grey54 	grey55
    grey56 	grey57 	grey58 	grey59 	grey6
    grey60 	grey61 	grey62 	grey63 	grey64
    grey65 	grey66 	grey67 	grey68 	grey69
    grey7 	grey70 	grey71 	grey72 	grey73
    grey74 	grey75 	grey76 	grey77 	grey78
    grey79 	grey8 	grey80 	grey81 	grey82
    grey83 	grey84 	grey85 	grey86 	grey87
    grey88 	grey89 	grey9 	grey90 	grey91
    grey92 	grey93 	grey94 	grey95 	grey96
    grey97 	grey98 	grey99 	honeydew 	honeydew1
    honeydew2 	honeydew3 	honeydew4 	hotpink 	hotpink1
    hotpink2 	hotpink3 	hotpink4 	indianred 	indianred1
    indianred2 	indianred3 	indianred4 	indigo 	invis
    ivory 	ivory1 	ivory2 	ivory3 	ivory4
    khaki 	khaki1 	khaki2 	khaki3 	khaki4
    lavender 	lavenderblush 	lavenderblush1 	lavenderblush2 	lavenderblush3
    lavenderblush4 	lawngreen 	lemonchiffon 	lemonchiffon1 	lemonchiffon2
    lemonchiffon3 	lemonchiffon4 	lightblue 	lightblue1 	lightblue2
    lightblue3 	lightblue4 	lightcoral 	lightcyan 	lightcyan1
    lightcyan2 	lightcyan3 	lightcyan4 	lightgoldenrod 	lightgoldenrod1
    lightgoldenrod2 	lightgoldenrod3 	lightgoldenrod4 	lightgoldenrodyellow 	lightgray
    lightgreen 	lightgrey 	lightpink 	lightpink1 	lightpink2
    lightpink3 	lightpink4 	lightsalmon 	lightsalmon1 	lightsalmon2
    lightsalmon3 	lightsalmon4 	lightseagreen 	lightskyblue 	lightskyblue1
    lightskyblue2 	lightskyblue3 	lightskyblue4 	lightslateblue 	lightslategray
    lightslategrey 	lightsteelblue 	lightsteelblue1 	lightsteelblue2 	lightsteelblue3
    lightsteelblue4 	lightyellow 	lightyellow1 	lightyellow2 	lightyellow3
    lightyellow4 	lime 	limegreen 	linen 	magenta
    magenta1 	magenta2 	magenta3 	magenta4 	maroon
    maroon1 	maroon2 	maroon3 	maroon4 	mediumaquamarine
    mediumblue 	mediumorchid 	mediumorchid1 	mediumorchid2 	mediumorchid3
    mediumorchid4 	mediumpurple 	mediumpurple1 	mediumpurple2 	mediumpurple3
    mediumpurple4 	mediumseagreen 	mediumslateblue 	mediumspringgreen 	mediumturquoise
    mediumvioletred 	midnightblue 	mintcream 	mistyrose 	mistyrose1
    mistyrose2 	mistyrose3 	mistyrose4 	moccasin 	navajowhite
    navajowhite1 	navajowhite2 	navajowhite3 	navajowhite4 	navy
    navyblue 	none 	oldlace 	olive 	olivedrab
    olivedrab1 	olivedrab2 	olivedrab3 	olivedrab4 	orange
    orange1 	orange2 	orange3 	orange4 	orangered
    orangered1 	orangered2 	orangered3 	orangered4 	orchid
    orchid1 	orchid2 	orchid3 	orchid4 	palegoldenrod
    palegreen 	palegreen1 	palegreen2 	palegreen3 	palegreen4
    paleturquoise 	paleturquoise1 	paleturquoise2 	paleturquoise3 	paleturquoise4
    palevioletred 	palevioletred1 	palevioletred2 	palevioletred3 	palevioletred4
    papayawhip 	peachpuff 	peachpuff1 	peachpuff2 	peachpuff3
    peachpuff4 	peru 	pink 	pink1 	pink2
    pink3 	pink4 	plum 	plum1 	plum2
    plum3 	plum4 	powderblue 	purple 	purple1
    purple2 	purple3 	purple4 	rebeccapurple 	red
    red1 	red2 	red3 	red4 	rosybrown
    rosybrown1 	rosybrown2 	rosybrown3 	rosybrown4 	royalblue
    royalblue1 	royalblue2 	royalblue3 	royalblue4 	saddlebrown
    salmon 	salmon1 	salmon2 	salmon3 	salmon4
    sandybrown 	seagreen 	seagreen1 	seagreen2 	seagreen3
    seagreen4 	seashell 	seashell1 	seashell2 	seashell3
    seashell4 	sienna 	sienna1 	sienna2 	sienna3
    sienna4 	silver 	skyblue 	skyblue1 	skyblue2
    skyblue3 	skyblue4 	slateblue 	slateblue1 	slateblue2
    slateblue3 	slateblue4 	slategray 	slategray1 	slategray2
    slategray3 	slategray4 	slategrey 	snow 	snow1
    snow2 	snow3 	snow4 	springgreen 	springgreen1
    springgreen2 	springgreen3 	springgreen4 	steelblue 	steelblue1
    steelblue2 	steelblue3 	steelblue4 	tan 	tan1
    tan2 	tan3 	tan4 	teal 	thistle
    thistle1 	thistle2 	thistle3 	thistle4 	tomato
    tomato1 	tomato2 	tomato3 	tomato4 	transparent
    turquoise 	turquoise1 	turquoise2 	turquoise3 	turquoise4
    violet 	violetred 	violetred1 	violetred2 	violetred3
    violetred4 	webgray 	webgreen 	webgrey 	webmaroon
    webpurple 	wheat 	wheat1 	wheat2 	wheat3
    wheat4 	white 	whitesmoke 	x11gray 	x11green
    x11grey 	x11maroon 	x11purple 	yellow 	yellow1
    yellow2 	yellow3 	yellow4 	yellowgreen""".split())

    for color in lowercase_colors:
        assert(color in check_colors)
        check_colors.remove(color)
    assert(not check_colors)

    with open("color_enum.txt", 'w') as file:
        for color in enum_colors:
            if color == "None":
                file.write(f'    #[default]\n')    
            file.write(f'    {color},\n')
    
    with open("color_str.txt", 'w') as file:
        for enum, color in zip(enum_colors, lowercase_colors):
            file.write(f'            Self::{enum} => "{color}",\n')

if __name__ == '__main__':
    compute_colors()
